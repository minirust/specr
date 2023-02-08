use crate::*;

#[test]
// this tests generates a dangling ptr by casting the int 42 to a pointer.
// then we deallocate the ptr, to obtain UB.
fn dealloc_inv_ptr() {
    let locals = [ <*const i32>::get_ptype() ];

    let b0 = block2(&[
        &live(0),
        &allocate(const_int::<usize>(4), const_int::<usize>(4), local(0), 1)
    ]);
    let b1 = block2(&[
        &assign(
            local(0),
            ptr_offset(
                load(local(0)),
                const_int::<usize>(1),
                InBounds::Yes
            )
        ),
        &deallocate(
            load(local(0)),
            const_int::<usize>(4),
            const_int::<usize>(4),
            2
        )
    ]);
    let b2 = block2(&[&exit()]);

    let f = function(Ret::No, 0, &locals, &[b0, b1, b2]);
    let p = program(&[f]);
    dump_program(&p);
    assert_ub(p, "deallocating with pointer not to the beginning of its allocation");
}
