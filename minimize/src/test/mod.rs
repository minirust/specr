use crate::*;

mod env;
use env::*;

#[test]
fn too_large_alloc() {
    fn program_alloc(bytes: Int) -> Program {
        let count = bytes;
        let array = Type::Array { elem: GcCow::new(Type::Bool), count };

        let l0 = LocalName(Name(0));
        let stmts = vec![
            Statement::StorageLive(l0),
            Statement::StorageDead(l0),
        ];
        let locals = vec![
            PlaceType {
                ty: array,
                align: Align::from_bytes(64),
            }
        ];
        program_from_statements(stmts, locals)
    }

    run_sequential(|| {
        let large = Int::from(2).pow(BasicMemory::PTR_SIZE.bits());
        assert_unwell(program_alloc(large));

        let small = Int::from(2);
        assert_stop(program_alloc(small));
    });
}

#[test]
fn double_live() {
    run_sequential(|| {
        let l0 = LocalName(Name(0));
        let stmts = vec![
            Statement::StorageLive(l0),
            Statement::StorageLive(l0),
        ];
        let locals = vec![
            PlaceType {
                ty: Type::Bool,
                align: Align::from_bytes(1),
            }
        ];
        let p = program_from_statements(stmts, locals);
        assert_unwell(p);
    });
}

#[test]
fn dead_before_live() {
    run_sequential(|| {
        let l0 = LocalName(Name(0));
        let stmts = vec![
            Statement::StorageDead(l0),
        ];
        let locals = vec![
            PlaceType {
                ty: Type::Bool,
                align: Align::from_bytes(1),
            }
        ];
        let p = program_from_statements(stmts, locals);
        assert_unwell(p);
    });
}

#[test]
fn uninit_read() {
    run_sequential(|| {
        let l0 = LocalName(Name(0));
        let l1 = LocalName(Name(1));
        let stmts = vec![
            Statement::StorageLive(l0),
            Statement::StorageLive(l1),
            Statement::Assign {
                destination: PlaceExpr::Local(l0),
                source: ValueExpr::Load {
                    destructive: false,
                    source: GcCow::new(PlaceExpr::Local(l1)),
                },
            },
        ];
        let pt = PlaceType {
            ty: Type::Bool,
            align: Align::from_bytes(1),
        };
        let locals = vec![pt, pt];

        let p = program_from_statements(stmts, locals);
        assert_ub(p, "load at type PlaceType { ty: Bool, align: Align { raw: Small(1) } } but the data in memory violates the validity invariant");
    });
}

// see https://github.com/rust-lang/miri/issues/845
#[test]
fn no_preserve_padding() {
    // Pair := Type::Tuple { ... }
    // Union := Type::Union { f0: Pair, f1: u32 }
    //
    // let _0: Union;
    // let _1: *const u8;
    // let _2: u8;
    //
    // _0.f1 = 0;
    // _1 = &raw _0.f0;
    // _1 = load(_1).offset(1)
    // _2 = *_1;

    run_sequential(|| {
        let pair_ty = Type::Tuple {
            fields: list![
                (Size::from_bytes(0), u8::get_type()),
                (Size::from_bytes(2), u16::get_type())
            ],
            size: Size::from_bits(32),
        };

        let union_ty = Type::Union {
            fields: list![
                (Size::ZERO, pair_ty),
                (Size::ZERO, u32::get_type())
            ],
            chunks: list![(Size::ZERO, Size::from_bytes(4))],
            size: Size::from_bytes(4),
        };
        let union_pty = PlaceType {
            ty: union_ty,
            align: Align::from_bytes(4),
        };

        let locals = vec![
            union_pty,
            <*const u8>::get_ptype(),
            <u8>::get_ptype()
        ];

        let stmts = vec![
            Statement::StorageLive(l(0)),
            Statement::StorageLive(l(1)),
            Statement::StorageLive(l(2)),
            Statement::Assign {
                destination: field(local(0), 1),
                source: const_int::<u32>(0),
            },
            Statement::Assign {
                destination: local(1),
                source: ValueExpr::AddrOf {
                    target: GcCow::new(field(local(0), 0)),
                    ptr_ty: PtrType::Raw { pointee: <u8>::get_layout() },
                },
            },
            Statement::Assign {
                destination: local(1),
                source: ValueExpr::BinOp {
                    operator: BinOp::PtrOffset { inbounds: true }, // TODO inbounds or not?
                    left: GcCow::new(load(local(1))),
                    right: GcCow::new(const_int::<u32>(1)),
                },
            },
            Statement::Assign {
                destination: local(2),
                source: load(deref(load(local(1)), <u8>::get_ptype())),
            },
        ];

        let p = program_from_statements(stmts, locals);
        dump_program(&p);
        assert_ub(p, "");
    });
}
