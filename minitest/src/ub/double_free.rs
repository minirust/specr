use crate::*;

#[test]
fn double_free() {
    let locals = vec![<*const i32>::get_ptype()];
    let n = const_int::<usize>(4);
    let b0 = block(&[live(0)], allocate(n, n, local(0), 1));
    let b1 = block(&[], deallocate(load(local(0)), n, n, 2));
    let b2 = block(&[], deallocate(load(local(0)), n, n, 3));
    let b3 = block(&[], exit());
    let f = function(Ret::No, 0, &locals, &[b0, b1, b2, b3]);
    let p = program(&[f]);
    assert_ub(p, "double-free");
}
