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
    format!("{:?}", t)
}

pub fn bbname_to_string(bb: BbName) -> String {
    format!("bb{}", bb.0.0)
}

pub fn fnname_to_string(fnname: FnName) -> String {
    format!("f{}", fnname.0.0)
}
