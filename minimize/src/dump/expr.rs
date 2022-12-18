use super::*;

pub fn place_expr_to_string(p: PlaceExpr) -> String {
    match p {
        PlaceExpr::Local(l) => localname_to_string(l),
        PlaceExpr::Deref { operand, .. } => {
            format!("*{}", value_expr_to_string(operand.get()))
        },
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

fn constant_to_string(c: Constant) -> String {
    match c {
        Constant::Int(int) => int.to_string(),
        Constant::Bool(b) => b.to_string(),
        Constant::Tuple(l) => {
            let l: Vec<_> = l.iter().map(constant_to_string).collect();
            let l = l.join(", ");

            format!("({l})")
        },
        c => format!("{:?}", c),
    }
}

pub fn value_expr_to_string(v: ValueExpr) -> String {
    match v {
        ValueExpr::Constant(c, _ty) => constant_to_string(c),
        ValueExpr::Load { destructive: _, source } => {
            let source = source.get();
            let source = place_expr_to_string(source);
            format!("load({source})")
        },
        ValueExpr::AddrOf { target, ptr_ty: PtrType::Raw { .. } } => {
            let target = target.get();
            let target = place_expr_to_string(target);
            format!("&raw {target}")
        },
        ValueExpr::AddrOf { target, ptr_ty: PtrType::Ref { mutbl, .. } } => {
            let target = target.get();
            let target = place_expr_to_string(target);
            let mutbl = match mutbl {
                Mutability::Mutable => "mut ",
                Mutability::Immutable => "",
            };
            format!("&{mutbl}{target}")
        },
        ValueExpr::AddrOf { target: _, ptr_ty: PtrType::Box { .. } } => {
            panic!("what? AddrOf with Box?")
        },
        ValueExpr::BinOp { operator: BinOp::Int(int_op, int_ty), left, right } => {
            let int_op = match int_op {
                BinOpInt::Add => "+",
                BinOpInt::Sub => "-",
                BinOpInt::Mul => "*",
                BinOpInt::Div => "/",
            };

            let int_ty = int_type_to_string(int_ty);
            let int_op = format!("{int_op}_{int_ty}");

            let l = value_expr_to_string(left.get());
            let r = value_expr_to_string(right.get());

            format!("{l} {int_op} {r}")
        },
        v => format!("{:?}", v)
    }
}

fn int_type_to_string(int_ty: IntType) -> String {
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
        t => format!("{:?}", t),
    }
}

pub fn bbname_to_string(bb: BbName) -> String {
    format!("bb{}", bb.0.0)
}

pub fn fnname_to_string(fnname: FnName) -> String {
    format!("f{}", fnname.0.0)
}
