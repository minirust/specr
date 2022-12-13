extern {
    fn print_i32(x: i32);
}

fn main() {
    let x = 30;
    unsafe { print_i32(foo(x-12).1) };
}

fn foo(x: i32) -> (i32, i32) {
    unsafe { print_i32(x) };
    (x+1, x+2)
}
