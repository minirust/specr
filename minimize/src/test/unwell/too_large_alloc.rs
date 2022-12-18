use crate::test::*;

#[test]
fn too_large_alloc() {
    run_sequential(|| {
        let ty = <[u8; usize::MAX/2+1]>::get_type();
        let pty = ptype(ty, align(1));

        let locals = vec![pty];
        let stmts = vec![live(0), dead(0)];

        let prog = small_program(&locals, &stmts);
        assert_unwell(prog);
    });
}
