use crate::test::*;

pub fn const_int<T: TypeConv>(int: impl Into<Int>) -> ValueExpr {
    ValueExpr::Constant(Constant::Int(int.into()), T::get_type())
}

pub fn const_bool(b: bool) -> ValueExpr {
    ValueExpr::Constant(Constant::Bool(b), Type::Bool)
}

// this gets ValueExprs instead of Constants to be compatible with the functions above.
pub fn const_tuple(args: &[ValueExpr], ty: Type) -> ValueExpr {
    let Type::Tuple { fields, .. } = ty else {
        panic!("const_tuple received non-tuple type!");
    };
    assert_eq!(fields.len(), args.len());
    
    let tuple = Constant::Tuple(args.iter().zip(fields).map(|(x, (_offset, field_ty))| {
        match x {
            ValueExpr::Constant(c, sub_ty) => {
                assert_eq!(*sub_ty, field_ty);

                *c
            },
            _ => panic!("const_tuple received non-const arg!"),
        }
    }).collect());

    ValueExpr::Constant(tuple, ty)
}

// non-destructive load.
pub fn load(p: PlaceExpr) -> ValueExpr {
    ValueExpr::Load {
        source: GcCow::new(p),
        destructive: false,
    }
}

pub fn load_destructive(p: PlaceExpr) -> ValueExpr {
    ValueExpr::Load {
        source: GcCow::new(p),
        destructive: true,
    }
}

pub fn addr_of(target: PlaceExpr, ptr_ty: PtrType) -> ValueExpr {
    ValueExpr::AddrOf {
        target: GcCow::new(target),
        ptr_ty,
    }
}

// TODO do BinOp & UnOp.

pub fn local(x: u32) -> PlaceExpr {
    PlaceExpr::Local(LocalName(Name(x)))
}

pub fn deref(operand: ValueExpr, ptype: PlaceType) -> PlaceExpr {
    PlaceExpr::Deref {
        operand: GcCow::new(operand),
        ptype,
    }
}

pub fn field(root: PlaceExpr, field: impl Into<Int>) -> PlaceExpr {
    PlaceExpr::Field {
        root: GcCow::new(root),
        field: field.into(),
    }
}

pub fn index(root: PlaceExpr, index: ValueExpr) -> PlaceExpr {
    PlaceExpr::Index {
        root: GcCow::new(root),
        index: GcCow::new(index),
    }
}
