use crate::test::*;

#[test]
fn dead_before_live() {
    run_sequential(|| {
        let locals = vec![ <bool>::get_ptype() ];
        let stmts = vec![dead(0)];
        let p = small_program(&locals, &stmts);
        assert_unwell(p);
    });
}
