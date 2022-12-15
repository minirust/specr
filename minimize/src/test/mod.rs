use crate::*;

mod build;
use build::*;

pub fn assert_ub(prog: Program, msg: &str) {
    assert_eq!(run_program(prog), Outcome::Ub(msg.to_string()));
}

pub fn assert_stop(prog: Program) {
    assert_eq!(run_program(prog), Outcome::Stop);
}

pub fn assert_unwell(prog: Program) {
    assert_eq!(run_program(prog), Outcome::Unwell);
}

/*
#[test]
fn too_large_alloc() {
    fn program_alloc(bytes: Int) -> Program {
        let count = bytes;
        let array = Type::Array { elem: GcCow::new(Type::Bool), count };

        let locals = vec![ptype(array, align(1))];
        let stmts = vec![live(0), dead(0)];
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
        let locals = vec![ <bool>::get_ptype() ];
        let stmts = vec![live(0), live(0)];
        let p = program_from_statements(stmts, locals);
        assert_unwell(p);
    });
}

#[test]
fn dead_before_live() {
    run_sequential(|| {
        let locals = vec![ <bool>::get_ptype() ];
        let stmts = vec![dead(0)];
        let p = program_from_statements(stmts, locals);
        assert_unwell(p);
    });
}

#[test]
fn uninit_read() {
    run_sequential(|| {
        let locals = vec![ <bool>::get_ptype(); 2];
        let stmts = vec![
            live(0),
            live(1),
            assign(
                local(0),
                load(local(1)),
            ),
        ];
        let p = program_from_statements(stmts, locals);
        assert_ub(p, "load at type PlaceType { ty: Bool, align: Align { raw: Small(1) } } but the data in memory violates the validity invariant");
    });
}

// see https://github.com/rust-lang/miri/issues/845
#[test]
fn no_preserve_padding() {
    // type Pair = (u8, u16);
    // union Union { f0: Pair, f1: u32 }
    //
    // let _0: Union;
    // let _1: Pair;
    // let _2: *const u8;
    // let _3: u8;
    //
    // _0.f1 = 0;
    // _1 = _0.f0;
    // _2 = &raw _1;
    // _2 = load(_2).offset(1)
    // _3 = *_2;

    run_sequential(|| {
        let pair_ty = Type::Tuple {
            fields: list![
                (size(0), u8::get_type()),
                (size(2), u16::get_type())
            ],
            size: size(4),
        };
        let pair_pty = ptype(pair_ty, align(2));

        let union_ty = Type::Union {
            fields: list![
                (size(0), pair_ty),
                (size(0), u32::get_type())
            ],
            chunks: list![(size(0), size(4))],
            size: size(4),
        };
        let union_pty = ptype(union_ty, align(4));

        let locals = vec![
            union_pty,
            pair_pty,
            <*const u8>::get_ptype(),
            <u8>::get_ptype(),
        ];

        let stmts = vec![
            live(0),
            live(1),
            live(2),
            live(3),
            assign(
                field(local(0), 1),
                const_int::<u32>(0)
            ),
            assign(
                local(1),
                load(field(local(0), 0))
            ),
            assign(
                local(2),
                ValueExpr::AddrOf {
                    target: GcCow::new(local(1)),
                    ptr_ty: PtrType::Raw { pointee: <u8>::get_layout() },
                },
            ),
            assign(
                local(2),
                ValueExpr::BinOp {
                    operator: BinOp::PtrOffset { inbounds: true }, // TODO inbounds or not?
                    left: GcCow::new(load(local(2))),
                    right: GcCow::new(const_int::<u32>(1)),
                }
            ),
            assign(
                local(3),
                load(deref(load(local(2)), <u8>::get_ptype())),
            ),
        ];

        let p = program_from_statements(stmts, locals);
        dump_program(&p);
        assert_ub(p, "load at type PlaceType { ty: Int(IntType { signed: Unsigned, size: Size { raw: Small(1) } }), align: Align { raw: Small(1) } } but the data in memory violates the validity invariant");
    });
}
*/
