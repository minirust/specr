extern crate intrinsics;
use intrinsics::*;
include!("../transmute.rs");

fn main() {
    unsafe {
        let x: i32 = 2;
        let i: usize = transmute(&x as *const _);
        let i = i + 4; // the i32 after `x` doesn't exist!
        let i: *const i32 = transmute(i);
        print(*i);
    }
}
