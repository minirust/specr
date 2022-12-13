extern {
    fn print_i64(x: i64);
}

fn main() {
    let mut x = 23i64;
    x += 3;
    x = double(x);
    unsafe { print_i64(x); }
}

fn double(x: i64) -> i64 {
    x * 2
}
