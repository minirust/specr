use crate::fmt::*;

pub fn ptype_to_string(place_ty: PlaceType, comptypes: &mut CompTypes) -> String {
    let ty_str = type_to_string(place_ty.ty, comptypes);
    let align = place_ty.align.bytes();
    format!("{ty_str}<align={align}>")
}

pub fn int_type_to_string(int_ty: IntType) -> String {
    let signed = match int_ty.signed {
        Signed => "i",
        Unsigned => "u",
    };
    let bits = int_ty.size.bits();

    format!("{signed}{bits}")
}

fn layout_to_string(layout: Layout) -> String {
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
fn get_comptype_index(ty: Type, comptypes: &mut CompTypes) -> usize {
    // check that `ty` is indeed a composite type.
    assert!(matches!(ty, Type::Union { .. } | Type::Tuple { .. }));

    match comptypes.iter().position(|x| *x == ty) {
        Some(i) => i,
        None => {
            let n = comptypes.len();
            comptypes.push(ty);
            n
        }
    }
}

pub fn type_to_string(t: Type, comptypes: &mut CompTypes) -> String {
    match t {
        Type::Int(int_ty) => int_type_to_string(int_ty),
        Type::Bool => String::from("bool"),
        Type::Ptr(PtrType::Ref {
            mutbl: Mutability::Mutable,
            pointee,
        }) => {
            let layout_str = layout_to_string(pointee);
            format!("&mut {layout_str}")
        },
        Type::Ptr(PtrType::Ref {
            mutbl: Mutability::Immutable,
            pointee,
        }) => {
            let layout_str = layout_to_string(pointee);
            format!("&{layout_str}")
        }
        Type::Ptr(PtrType::Box { pointee }) => {
            let layout_str = layout_to_string(pointee);
            format!("Box<{layout_str}>")
        }
        Type::Ptr(PtrType::Raw { pointee }) => {
            let layout_str = layout_to_string(pointee);
            format!("*{layout_str}")
        }
        Type::Ptr(PtrType::FnPtr) => String::from("fn()"),
        Type::Tuple { .. } | Type::Union { .. } => {
            comptype_to_string(get_comptype_index(t, comptypes))
        }
        Type::Array { elem, count } => {
            let elem = type_to_string(elem.extract(), comptypes);
            format!("[{elem}; {count}]")
        }
        Type::Enum { .. } => panic!("enums are unsupported!"),
    }
}

pub fn fmt_comptype(i: usize, t: Type, comptypes: &mut CompTypes) -> String {
    let (keyword, fields, opt_chunks, size) = match t {
        Type::Tuple { fields, size } => ("tuple", fields, None, size),
        Type::Union {
            chunks,
            fields,
            size,
        } => ("union", fields, Some(chunks), size),
        _ => panic!("not a supported composite type!"),
    };
    let ct = comptype_to_string(i);
    let size = size.bytes();
    let mut s = format!("{keyword} {ct} ({size} bytes) {{\n");
    for (offset, f) in fields {
        let offset = offset.bytes();
        let ty = type_to_string(f, comptypes);
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
