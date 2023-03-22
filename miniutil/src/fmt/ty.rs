use super::*;

// A "composite" type, namely a union or tuple.
// Composite types will not be printed inline,
// but instead they will be printed above the functions.
// During formatting, the list of composite types we encounter will be stored in `comptypes`.
#[derive(PartialEq, Eq, Clone, Copy)]
pub(super) struct CompType(pub(super) Type);

// An index into `comptypes`.
// will be formatted as `T{idx}`
pub(super) struct CompTypeIndex {
    pub(super) idx: usize,
}

pub(super) fn fmt_ptype(place_ty: PlaceType, comptypes: &mut Vec<CompType>) -> String {
    let ty_str = fmt_type(place_ty.ty, comptypes);
    let align = place_ty.align.bytes();
    format!("{ty_str}<align={align}>")
}

pub(super) fn fmt_int_type(int_ty: IntType) -> String {
    let signed = match int_ty.signed {
        Signed => "i",
        Unsigned => "u",
    };
    let bits = int_ty.size.bits();

    format!("{signed}{bits}")
}

fn fmt_layout(layout: Layout) -> String {
    let size = layout.size.bytes();
    let align = layout.align.bytes();
    let uninhab_str = match layout.inhabited {
        true => "",
        false => ", uninhabited",
    };
    format!("layout(size={size}, align={align}{uninhab_str})")
}

// `ty` is a composite type.
// Gives the index of `ty` within `comptypes`.
// This adds `ty` to `comptypes` if it's not yet in there.
fn get_comptype_index(ty: Type, comptypes: &mut Vec<CompType>) -> CompTypeIndex {
    // check that `ty` is indeed a composite type.
    assert!(matches!(ty, Type::Union { .. } | Type::Tuple { .. }));
    let comp_ty = CompType(ty);
    let idx = match comptypes.iter().position(|x| *x == comp_ty) {
        Some(i) => i,
        None => {
            let n = comptypes.len();
            comptypes.push(comp_ty);
            n
        }
    };

    CompTypeIndex { idx }
}

pub(super) fn fmt_type(t: Type, comptypes: &mut Vec<CompType>) -> String {
    match t {
        Type::Int(int_ty) => fmt_int_type(int_ty),
        Type::Bool => String::from("bool"),
        Type::Ptr(PtrType::Ref {
            mutbl: Mutability::Mutable,
            pointee,
        }) => {
            let layout_str = fmt_layout(pointee);
            format!("&mut {layout_str}")
        }
        Type::Ptr(PtrType::Ref {
            mutbl: Mutability::Immutable,
            pointee,
        }) => {
            let layout_str = fmt_layout(pointee);
            format!("&{layout_str}")
        }
        Type::Ptr(PtrType::Box { pointee }) => {
            let layout_str = fmt_layout(pointee);
            format!("Box<{layout_str}>")
        }
        Type::Ptr(PtrType::Raw { pointee }) => {
            let layout_str = fmt_layout(pointee);
            format!("*{layout_str}")
        }
        Type::Ptr(PtrType::FnPtr) => String::from("fn()"),
        Type::Tuple { .. } | Type::Union { .. } => {
            let comptype_index = get_comptype_index(t, comptypes);
            fmt_comptype_index(comptype_index)
        }
        Type::Array { elem, count } => {
            let elem = fmt_type(elem.extract(), comptypes);
            format!("[{elem}; {count}]")
        }
        Type::Enum { .. } => panic!("enums are unsupported!"),
    }
}

pub(super) fn fmt_comptype(i: CompTypeIndex, t: CompType, comptypes: &mut Vec<CompType>) -> String {
    let (keyword, fields, opt_chunks, size) = match t.0 {
        Type::Tuple { fields, size } => ("tuple", fields, None, size),
        Type::Union {
            chunks,
            fields,
            size,
        } => ("union", fields, Some(chunks), size),
        _ => panic!("not a supported composite type!"),
    };
    let ct = fmt_comptype_index(i);
    let size = size.bytes();
    let mut s = format!("{keyword} {ct} ({size} bytes) {{\n");
    for (offset, f) in fields {
        let offset = offset.bytes();
        let ty = fmt_type(f, comptypes);
        s += &format!("  at byte {offset}: {ty},\n");
    }
    if let Some(chunks) = opt_chunks {
        for (offset, size) in chunks {
            let offset = offset.bytes();
            let size = size.bytes();
            s += &format!("  chunk(at={offset}, size={size}),\n");
        }
    }
    s += "}\n\n";
    s
}
