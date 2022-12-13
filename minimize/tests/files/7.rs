extern {
    fn print_i32(x: i32);
}

fn main() {
    let mut x = 2;
    let y = &mut x as *mut i32;
    unsafe { *y = 3; }
    unsafe { print_i32(x) };
}
