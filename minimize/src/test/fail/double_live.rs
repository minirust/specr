use crate::test::*;

#[test]
fn double_live() {
    run_sequential(|| {
        let locals = vec![ <bool>::get_ptype() ];
        let stmts = vec![live(0), live(0)];
        let p = small_program(&locals, &stmts);
        assert_unwell(p);
    });
}
