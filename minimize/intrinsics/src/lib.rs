// Minimize will replace each call to this function by a `CallIntrinsic`.
pub fn print<T: Show>(t: T) {
    println!("{}", t.show());
}

pub trait Show {
    fn show(&self) -> String;
}

impl Show for i32 { fn show(&self) -> String { self.to_string() } }
impl Show for i64 { fn show(&self) -> String { self.to_string() } }
