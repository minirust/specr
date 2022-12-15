include!("../helper/transmute.rs");

fn main() {
    unsafe {
        let _x: i32 = *transmute::<usize, *const i32>(8);
    }
}
