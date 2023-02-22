use crate::*;

pub fn int_type_to_string(int_ty: IntType) -> String {
    let signed = match int_ty.signed {
        Signed => "i",
        Unsigned => "u",
    };
    let bits = int_ty.size.bits();

    format!("{signed}{bits}")
}

pub fn type_to_string(t: Type) -> String {
    match t {
        Type::Int(int_ty) => int_type_to_string(int_ty),
        Type::Bool => String::from("bool"),
        Type::Ptr(PtrType::Ref { mutbl: Mutability::Mutable, .. }) => String::from("&mut _"),
        Type::Ptr(PtrType::Ref { mutbl: Mutability::Immutable, .. }) => String::from("&_"),
        Type::Ptr(PtrType::Box { .. }) => String::from("Box<_>"),
        Type::Ptr(PtrType::Raw { .. }) => String::from("*_"),
        Type::Tuple { fields, .. } => {
            let fields: Vec<_> = fields.iter().map(|(_, ty)| type_to_string(ty)).collect();
            let fields = fields.join(", ");

            format!("({fields})")
        },
        Type::Array { elem, count } => {
            let elem = type_to_string(elem.get());
            format!("[{}; {}]", elem, count)
        },
        Type::Union { .. } => format!("{:?}", t),
        Type::Enum { .. } => panic!("enums are unsupported!"),
    }
}
