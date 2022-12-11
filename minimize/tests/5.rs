extern crate intrinsics;
use intrinsics::print;

fn main() {
    let mut x = 2;
    let y = &mut x;
    *y = 3;
    print(x);
}
