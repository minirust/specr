extern crate intrinsics;
use intrinsics::*;
include!("../conv.rs");

fn main() {
    unsafe {
        let x = 2;
        let i: i32 = *i_to_p(ref_to_i(&x) + 1);
        print(i);
    }
}
