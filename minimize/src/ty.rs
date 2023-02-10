use crate::*;

pub fn layout_of<'tcx>(ty: rs::Ty<'tcx>, tcx: rs::TyCtxt<'tcx>) -> Layout {
    let a = rs::ParamEnv::empty().and(ty);
    let layout = tcx.layout_of(a).unwrap().layout;
    let size = translate_size(layout.size());
    let align = translate_align(layout.align().pref);
    let inhabited = !layout.abi().is_uninhabited();

    Layout {
        size,
        align,
        inhabited,
    }
}

pub fn translate_mutbl(mutbl: rs::Mutability) -> Mutability {
    match mutbl {
        rs::Mutability::Mut => Mutability::Mutable,
        rs::Mutability::Not => Mutability::Immutable,
    }
}

// TODO it would be nicer to access this from Minirust itself.
fn ty_size(ty: Type) -> Size {
    use Type::*;
    match ty {
        Int(int_type) => int_type.size,
        Bool => Size::from_bytes_const(1),
        Ptr(_) => BasicMemory::PTR_SIZE,
        Tuple { size, .. } | Union { size, .. } | Enum { size, .. } => size,
        Array { elem, count } => ty_size(elem.get()) * count,
    }
}

// The `markers` object stores a bool for each byte within the size of a union.
// Such a bool is `true` if the corresponding byte should be part of a chunk (i.e. it contains actual data),
// and it's false it this byte is just padding.
//
// marks any non-padding bytes used by `ty` as `true`.
fn mark_used_bytes(ty: Type, markers: &mut [bool]) {
    match ty {
        Type::Int(int_ty) => mark_size(int_ty.size, markers),
        Type::Bool => mark_size(Size::from_bytes_const(1), markers),
        Type::Ptr(_) => mark_size(BasicMemory::PTR_SIZE, markers),
        Type::Tuple { fields, .. } | Type::Union { fields, .. } => {
            for (offset, ty) in fields {
                let offset = offset.bytes().try_to_usize().unwrap();
                mark_used_bytes(ty, &mut markers[offset..]);
            }
        },
        Type::Array { elem, count } => {
            let elem = elem.get();
            for i in Int::ZERO..count {
                let offset = i * ty_size(elem);
                let offset = offset.bytes().try_to_usize().unwrap();
                mark_used_bytes(elem, &mut markers[offset..]);
            }
        }
        Type::Enum { .. } => panic!("unsupported!"),
    }
}

// marks all bytes from 0..size as true.
fn mark_size(size: Size, markers: &mut [bool]) {
    for i in Int::ZERO..size.bytes() {
        let i = i.try_to_usize().unwrap();
        markers[i] = true;
    }
}

// see https://github.com/rust-lang/unsafe-code-guidelines/issues/354
fn calc_chunks(fields: Fields, size: Size) -> List<(Size, Size)> {
    let s = size.bytes().try_to_usize().unwrap();
    let mut markers = vec![false; s];
    for (offset, ty) in fields {
        let offset = offset.bytes().try_to_usize().unwrap();
        mark_used_bytes(ty, &mut markers[offset..]);
    }

    let mut chunks = Vec::new();
    let mut current_chunk_start: Option<usize> = None;

    // this garantees that markers stops with false,
    // and the last chunk will be added.
    markers.push(false);

    for (i, b) in markers.iter().enumerate() {
        match (b, &current_chunk_start) {
            (true, None) => {
                current_chunk_start = Some(i);
            },
            (false, Some(s)) => {
                let start = Size::from_bytes(*s).unwrap();
                let length = Size::from_bytes(i - *s).unwrap();
                chunks.push((start, length));
                current_chunk_start = None;
            },
            _ => {},
        }
    }

    chunks.into_iter().collect()
}

pub fn translate_ty<'tcx>(ty: rs::Ty<'tcx>, tcx: rs::TyCtxt<'tcx>) -> Type {
    match ty.kind() {
        rs::TyKind::Bool => Type::Bool,
        rs::TyKind::Int(int_ty) => Type::Int(translate_int_ty(int_ty)),
        rs::TyKind::Uint(uint_ty) => Type::Int(translate_uint_ty(uint_ty)),
        rs::TyKind::Tuple(ts) => {
            let a = rs::ParamEnv::empty().and(ty);
            let layout = tcx.layout_of(a).unwrap().layout;
            let size = translate_size(layout.size());

            let fields = ts.iter()
                           .enumerate()
                           .map(|(i, t)| {
                                let t = translate_ty(t, tcx);
                                let offset = layout.fields().offset(i);
                                let offset = translate_size(offset);

                                (offset, t)
                           }).collect();

            Type::Tuple {
                fields,
                size,
            }
        },
        rs::TyKind::Adt(adt_def, sref) if adt_def.is_struct() => {
            let (fields, size) = translate_adt_fields(ty, *adt_def, sref, tcx);

            Type::Tuple {
                fields,
                size,
            }
        },
        rs::TyKind::Adt(adt_def, sref) if adt_def.is_union() => {
            let (fields, size) = translate_adt_fields(ty, *adt_def, sref, tcx);
            let chunks = calc_chunks(fields, size);

            Type::Union {
                fields,
                size,
                chunks,
            }
        },
        rs::TyKind::Adt(adt_def, _) if adt_def.is_box() => {
            let ty = ty.boxed_ty();
            let pointee = layout_of(ty, tcx);
            Type::Ptr(PtrType::Box { pointee })
        },
        rs::TyKind::Ref(_, ty, mutbl) => {
            let pointee = layout_of(*ty, tcx);
            let mutbl = translate_mutbl(*mutbl);
            Type::Ptr(PtrType::Ref { pointee, mutbl } )
        },
        rs::TyKind::RawPtr(rs::TypeAndMut { ty, mutbl: _ }) => {
            let pointee = layout_of(*ty, tcx);
            Type::Ptr(PtrType::Raw { pointee } )
        },
        rs::TyKind::Array(ty, c) => {
            let count = Int::from(c.eval_usize(tcx, rs::ParamEnv::empty()));
            let elem = GcCow::new(translate_ty(*ty, tcx));
            Type::Array { elem, count }
        },
        x => {
            dbg!(x);
            todo!()
        }
    }
}

fn translate_adt_fields<'tcx>(ty: rs::Ty<'tcx>, adt_def: rs::AdtDef<'tcx>, sref: rs::SubstsRef<'tcx>, tcx: rs::TyCtxt<'tcx>) -> (Fields, Size) {
    let a = rs::ParamEnv::empty().and(ty);
    let layout = tcx.layout_of(a).unwrap().layout;
    let fields = adt_def
       .all_fields()
       .enumerate()
       .map(|(i, field)| {
            let ty = field.ty(tcx, sref);
            let ty = translate_ty(ty, tcx);
            let offset = layout.fields().offset(i);
            let offset = translate_size(offset);

            (offset, ty)
       }).collect();
    let size = translate_size(layout.size());

    (fields, size)
}

fn translate_int_ty(int_ty: &rs::IntTy) -> IntType {
    use rs::IntTy::*;

    let size = match int_ty {
        Isize => 8, // this is fixed as 8, to be compatible with BasicMemory.
        I8 => 1,
        I16 => 2,
        I32 => 4,
        I64 => 8,
        I128 => 16,
    };

    let signed = Signedness::Signed;
    let size = Size::from_bytes_const(size);
    IntType { signed, size }
}

fn translate_uint_ty(uint_ty: &rs::UintTy) -> IntType {
    use rs::UintTy::*;

    let size = match uint_ty {
        Usize => 8, // this is fixed as 8, to be compatible with BasicMemory.
        U8 => 1,
        U16 => 2,
        U32 => 4,
        U64 => 8,
        U128 => 16,
    };

    let signed = Signedness::Unsigned;
    let size = Size::from_bytes_const(size);
    IntType { signed, size }
}

fn translate_size(size: rs::Size) -> Size {
    Size::from_bytes_const(size.bytes())
}
