use crate::test::*;

#[test]
fn type_mismatch() {
    run_sequential(|| {
        let locals = &[<i32>::get_ptype()];
        let stmts = &[
            live(0),
            assign(
                local(0),
                const_int::<u32>(0),
            ),
        ];
        let p = small_program(locals, stmts);
        assert_unwell(p);
    });
}
