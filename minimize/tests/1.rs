extern {
    fn print_i32(x: i32);
}

fn main() {
    unsafe { print_i32(42) };
}
