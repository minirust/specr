extern {
    fn print_i32(x: i32);
}

fn main() {
    let x = 2;
    let y = &x as *const i32;
    unsafe { print_i32(*y) };
}
