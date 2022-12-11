extern crate intrinsics;
use intrinsics::print;

fn main() {
    let x = 2;
    let y = &x;
    print(*y);
}
