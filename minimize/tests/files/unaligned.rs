extern crate intrinsics;
use intrinsics::*;
include!("../transmute.rs");

fn main() {
    unsafe {
        let x: i32 = 2;
        let i: usize = transmute(&x as *const _);
        let i = i + 1; // alignment is now off!
        let i: *const i32 = transmute(i);
        print(*i);
    }
}
