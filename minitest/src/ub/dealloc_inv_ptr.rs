use crate::*;

#[test]
// this tests generates a dangling ptr by casting the int 42 to a pointer.
// then we deallocate the ptr, to obtain UB.
fn dealloc_inv_ptr() {
    let union_ty = union_ty(&[
            (size(0), <usize>::get_type()),
            (size(0), <*const i32>::get_type()),
        ], size(8));
    let union_pty = ptype(union_ty, align(8));

    let locals = [ union_pty ];

    let b0 = block2(&[
        &live(0),
        &assign(
            field(local(0), 0),
            const_int::<usize>(42)
        ),
        &deallocate(
            load(field(local(0), 1)),
            const_int::<usize>(4), // size
            const_int::<usize>(4), // align
            1,
        )
    ]);
    let b1 = block2(&[&exit()]);

    let f = function(Ret::No, 0, &locals, &[b0, b1]);
    let p = program(&[f]);
    dump_program(&p);
    assert_ub(p, "deallocating invalid pointer");
}
