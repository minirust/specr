use crate::*;

#[test]
fn use_after_free() {
    let locals = vec![<*const i32>::get_ptype()];
    let n = const_int::<usize>(4);
    let b0 = block(&[live(0)], allocate(n, n, local(0), 1));
    let b1 = block(&[], deallocate(load(local(0)), n, n, 2));
    let b2 = block(&[
        assign( // write to ptr after dealloc!
            deref(load(local(0)), <i32>::get_ptype()),
            const_int::<i32>(42),
        )
    ], exit());
    let f = function(Ret::No, 0, &locals, &[b0, b1, b2]);
    let p = program(&[f]);
    assert_ub(p, "memory accessed after deallocation");
}
