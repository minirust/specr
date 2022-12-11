extern crate intrinsics;
use intrinsics::print;

fn main() {
    let x = 2;
    let y = &x as *const i32;
    print(unsafe { *y });
}
