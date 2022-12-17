use crate::test::*;

#[test]
fn too_large_alloc() {
    fn program_alloc(bytes: Int) -> Program {
        let count = bytes;
        let ty = array_ty(bool_ty(), count);

        let locals = vec![ptype(ty, align(1))];
        let stmts = vec![live(0), dead(0)];
        small_program(&locals, &stmts)
    }

    run_sequential(|| {
        let large = Int::from(2).pow(BasicMemory::PTR_SIZE.bits());
        assert_unwell(program_alloc(large));

        let small = Int::from(2);
        assert_stop(program_alloc(small));
    });
}
