use crate::test::*;

#[test]
fn call_missing_fn() {
    run_sequential(|| {
        let fn_id = 42;
        let b = block(&[], call(fn_id, &[], None, None));
        let main_fn = function(Ret::No, 0, &[], &[b]);
        let p = program(&[main_fn]);
        assert_ub(p, "calling non-existing function");
    });
}
