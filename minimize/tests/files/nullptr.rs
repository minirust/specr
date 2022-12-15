include!("../transmute.rs");

fn main() {
    unsafe {
        let _x = *transmute::<usize, *const i32>(0);
    }
}
