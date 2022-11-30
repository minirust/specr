use crate::*;
use crate::mini::*;

pub fn place_expr_to_string(p: PlaceExpr) -> String {
    match p {
        PlaceExpr::Local(l) => localname_to_string(l),
        PlaceExpr::Deref { .. } => format!("{:?}", p),
        PlaceExpr::Field { root, field } => {
            let root = root.get();
            format!("{}.{}", place_expr_to_string(root), field)
        },
        PlaceExpr::Index { root, index } => {
            let root = root.get();
            let index = index.get();
            format!("{}[{}]", place_expr_to_string(root), value_expr_to_string(index))
        },
    }
}

pub fn localname_to_string(l: LocalName) -> String {
    format!("_{}", l.0.0)
}

pub fn value_expr_to_string(v: ValueExpr) -> String {
    format!("{:?}", v)
}

pub fn type_to_string(t: Type) -> String {
    match t {
        Type::Int(int_ty) => {
            let signed = match int_ty.signed {
                Signed => "i",
                Unsigned => "u",
            };
            let bits = specr::hidden::int_to_usize(int_ty.size.bits());

            format!("{signed}{bits}")
        },
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
        t => format!("{:?}", t),
    }
}

pub fn bbname_to_string(bb: BbName) -> String {
    format!("bb{}", bb.0.0)
}

pub fn fnname_to_string(fnname: FnName) -> String {
    format!("f{}", fnname.0.0)
}
