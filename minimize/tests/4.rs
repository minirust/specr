extern {
    fn print_i32(x: i32);
}

fn main() {
    let x = 2;
    let y = &x;
    unsafe { print_i32(*y); }
}
