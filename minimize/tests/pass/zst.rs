include!("../helper/transmute.rs");

fn main() {
    unsafe {
        let _x: () = *transmute::<usize, *const ()>(16);
    }
}
