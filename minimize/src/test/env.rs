use crate::test::*;

pub fn assert_ub(prog: Program, msg: &str) {
    assert_eq!(run_program(prog), Outcome::Ub(msg.to_string()));
}

pub fn assert_stop(prog: Program) {
    assert_eq!(run_program(prog), Outcome::Stop);
}

pub fn assert_unwell(prog: Program) {
    assert_eq!(run_program(prog), Outcome::Unwell);
}

pub fn function_from_statements(stmts: Vec<Statement>, local_types: Vec<PlaceType>) -> Function {
    let bb = BasicBlock {
        statements: stmts.into_iter().collect(),
        terminator: Terminator::CallIntrinsic {
            intrinsic: Intrinsic::Exit,
            arguments: List::new(),
            ret: None,
            next_block: None,
        },
    };
    
    let mut locals = Map::new();
    for (i, l) in local_types.into_iter().enumerate() {
        let lname = LocalName(Name(i as _));
        locals.insert(lname, l);
    }

    let bbname = BbName(Name(0));
    let mut blocks = Map::new();
    blocks.insert(bbname, bb);
    
    Function {
        locals,
        args: List::new(),
        ret: None,
        blocks,
        start: bbname,
    }
}

pub fn program_from_statements(stmts: Vec<Statement>, locals: Vec<PlaceType>) -> Program {
    let f = function_from_statements(stmts, locals);

    let mut functions = Map::new();
    let start = FnName(Name(0));
    functions.insert(start, f);
    Program {
        functions,
        start,
    }
}

pub trait TypeConv {
    fn get_type() -> Type;
    fn get_align() -> Align;
    fn get_size() -> Size;

    fn get_ptype() -> PlaceType {
        PlaceType {
            ty: Self::get_type(),
            align: Self::get_align(),
        }
    }

    fn get_layout() -> Layout {
        Layout {
            size: Self::get_size(),
            align: Self::get_align(),
            inhabited: true, // currently there are no uninhabited types in minirust; Type::Enum is not yet supported!
        }
    }
}

macro_rules! type_conv_impl {
    ($ty:ty, $signed:expr, $size:expr, $align:expr) => {
        impl TypeConv for $ty {
            fn get_type() -> Type {
                Type::Int(IntType { signed: $signed, size: Size::from_bytes($size)})
            }
            fn get_align() -> Align {
                Align::from_bytes($align)
            }
            fn get_size() -> Size {
                Size::from_bytes($size)
            }
        }
    }
}

type_conv_impl!(u8, Unsigned, 1, 1);
type_conv_impl!(u16, Unsigned, 2, 2);
type_conv_impl!(u32, Unsigned, 4, 4);
type_conv_impl!(u64, Unsigned, 8, 8);
type_conv_impl!(u128, Unsigned, 16, 8);

type_conv_impl!(i8, Signed, 1, 1);
type_conv_impl!(i16, Signed, 2, 2);
type_conv_impl!(i32, Signed, 4, 4);
type_conv_impl!(i64, Signed, 8, 8);
type_conv_impl!(i128, Signed, 16, 8);

impl<T: TypeConv> TypeConv for *const T {
    fn get_type() -> Type {
        Type::Ptr(PtrType::Raw { pointee: T::get_layout() })
    }
    fn get_align() -> Align {
        Align::from_bytes(8)
    }
    fn get_size() -> Size {
        Size::from_bytes(8)
    }
}

pub fn f(x: u32) -> FnName { FnName(Name(x)) }
pub fn bb(x: u32) -> BbName { BbName(Name(x)) }
pub fn l(x: u32) -> LocalName { LocalName(Name(x)) }

pub fn local(x: u32) -> PlaceExpr {
    PlaceExpr::Local(l(x))
}

pub fn const_int<T: TypeConv>(i: i64) -> ValueExpr {
    let c = Constant::Int(i.into());
    let ty = T::get_type();

    ValueExpr::Constant(c, ty)
}

// is non-destructive.
pub fn load(source: PlaceExpr) -> ValueExpr {
    ValueExpr::Load {
        source: GcCow::new(source),
        destructive: false,
    }
}

pub fn field(root: PlaceExpr, i: u32) -> PlaceExpr {
    PlaceExpr::Field {
        root: GcCow::new(root),
        field: i.into(),
    }
}

pub fn deref(v: ValueExpr, ptype: PlaceType) -> PlaceExpr {
    PlaceExpr::Deref {
        operand: GcCow::new(v),
        ptype,
    }
}

pub fn align(x: u32) -> Align {
    Align::from_bytes(x)
}

pub fn size(x: u32) -> Size {
    Size::from_bytes(x)
}

pub fn assign(x: PlaceExpr, y: ValueExpr) -> Statement {
    Statement::Assign {
        destination: x,
        source: y,
    }
}

pub fn ptype(ty: Type, align: Align) -> PlaceType {
    PlaceType { ty, align }
}
