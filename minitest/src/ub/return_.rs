use crate::*;

#[test]
fn return_success() {
    let other_f = {
        let locals = [<()>::get_ptype()];
        let b0 = block2(&[&return_()]);

        function(Ret::Yes, 0, &locals, &[b0])
    };

    let locals = [<()>::get_ptype()];

    let b0 = block2(&[
        &live(0),
        &call(1, &[], Some(local(0)), Some(1))
    ]);
    let b1 = block2(&[&exit()]);

    let f = function(Ret::No, 0, &locals, &[b0, b1]);
    let p = program(&[f, other_f]);
    dump_program(&p);
    assert_stop(p);
}

#[test]
fn return_no_local() {
    let other_f = {
        let b0 = block2(&[&return_()]);

        function(Ret::No, 0, &[], &[b0])
    };

    let locals = [<()>::get_ptype()];

    let b0 = block2(&[
        &live(0),
        &call(1, &[], Some(local(0)), Some(1))
    ]);
    let b1 = block2(&[&exit()]);

    let f = function(Ret::No, 0, &locals, &[b0, b1]);
    let p = program(&[f, other_f]);
    dump_program(&p);
    assert_ub(p, "Return from a function that does not have a return local");
}

#[test]
fn return_no_next() {
    let other_f = {
        let locals = [<()>::get_ptype()];
        let b0 = block2(&[&return_()]);

        function(Ret::Yes, 0, &locals, &[b0])
    };

    let locals = [<()>::get_ptype()];

    let b0 = block2(&[
        &live(0),
        &call(1, &[], Some(local(0)), None)
    ]);

    let f = function(Ret::No, 0, &locals, &[b0]);
    let p = program(&[f, other_f]);
    dump_program(&p);
    assert_ub(p, "Return from a function where caller did not specify next block");
}
