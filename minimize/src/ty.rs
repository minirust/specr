use crate::*;

// TODO the ParamEnv might need to be an argument to `layout_of` in the future.
pub fn layout_of<'tcx>(ty: rs::Ty<'tcx>, tcx: rs::TyCtxt<'tcx>) -> mini::Layout {
    let a = rs::ParamEnv::empty().and(ty);
    let layout = tcx.layout_of(a).unwrap().layout;
    let size = translate_size(layout.size());
    let align = translate_align(layout.align().pref);
    let inhabited = !layout.abi().is_uninhabited();

    mini::Layout {
        size,
        align,
        inhabited,
    }
}

#[allow(unused)]
pub fn size_of<'tcx>(ty: rs::Ty<'tcx>, tcx: rs::TyCtxt<'tcx>) -> specr::Size {
    let a = rs::ParamEnv::empty().and(ty);
    let layout = tcx.layout_of(a).unwrap().layout;

    translate_size(layout.size())
}

pub fn translate_mutbl(mutbl: rs::Mutability) -> mini::Mutability {
    match mutbl {
        rs::Mutability::Mut => mini::Mutability::Mutable,
        rs::Mutability::Not => mini::Mutability::Immutable,
    }
}

pub fn translate_ty<'tcx>(ty: rs::Ty<'tcx>, tcx: rs::TyCtxt<'tcx>) -> mini::Type {
    match ty.kind() {
        rs::TyKind::Bool => mini::Type::Bool,
        rs::TyKind::Int(int_ty) => mini::Type::Int(translate_int_ty(int_ty)),
        rs::TyKind::Uint(uint_ty) => mini::Type::Int(translate_uint_ty(uint_ty)),
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

            mini::Type::Tuple {
                fields,
                size,
            }
        },

        // TODO support generics
        rs::TyKind::Adt(adt_def, sref) if adt_def.is_struct() => {
            let a = rs::ParamEnv::empty().and(ty);
            let layout = tcx.layout_of(a).unwrap().layout;
            let size = translate_size(layout.size());

            let fields = adt_def.all_fields()
                           .enumerate()
                           .map(|(i, field)| {
                                let ty = field.ty(tcx, sref);
                                let ty = translate_ty(ty, tcx);
                                let offset = layout.fields().offset(i);
                                let offset = translate_size(offset);

                                (offset, ty)
                           }).collect();

            mini::Type::Tuple {
                fields,
                size,
            }
        },
        rs::TyKind::Adt(adt_def, _) if adt_def.is_box() => {
            let ty = ty.boxed_ty();
            let pointee = layout_of(ty, tcx);
            mini::Type::Ptr(mini::PtrType::Box { pointee })
        },
        rs::TyKind::Ref(_, ty, mutbl) => {
            let pointee = layout_of(*ty, tcx);
            let mutbl = translate_mutbl(*mutbl);
            mini::Type::Ptr(mini::PtrType::Ref { pointee, mutbl } )
        },
        rs::TyKind::RawPtr(rs::TypeAndMut { ty, mutbl: _ }) => {
            let pointee = layout_of(*ty, tcx);
            mini::Type::Ptr(mini::PtrType::Raw { pointee } )
        },
        x => {
            dbg!(x);
            todo!()
        }
    }
}

fn translate_int_ty(int_ty: &rs::IntTy) -> mini::IntType {
    use rs::IntTy::*;

    let size = match int_ty {
        // TODO look at TargetDataLayout for Isize.
        Isize => 8,
        I8 => 1,
        I16 => 2,
        I32 => 4,
        I64 => 8,
        I128 => 16,
    };

    let signed = mini::Signedness::Signed;
    let size = specr::Size::from_bytes(size);
    mini::IntType { signed, size }
}

fn translate_uint_ty(uint_ty: &rs::UintTy) -> mini::IntType {
    use rs::UintTy::*;

    let size = match uint_ty {
        Usize => 8, // TODO this is not generally 8.
        U8 => 1,
        U16 => 2,
        U32 => 4,
        U64 => 8,
        U128 => 16,
    };

    let signed = mini::Signedness::Unsigned;
    let size = specr::Size::from_bytes(size);
    mini::IntType { signed, size }
}

fn translate_size(size: rs::Size) -> specr::Size {
    specr::Size::from_bytes(size.bytes())
}
