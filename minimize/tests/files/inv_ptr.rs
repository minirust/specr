extern crate intrinsics;
use intrinsics::*;

union A {
    p: *const i32,
    i: usize,
}

fn main() {
    unsafe {
        let x = 2;
        let mut a = A { p: &x as *const _ };
        a.i += 4; // alignment is correct, but pointer still invalid.
        let ptr = a.p;
        let i: i32 = *ptr;
        print(i);
    }
}
