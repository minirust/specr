use crate::*;

#[const_trait]
pub trait ToInt {
    fn to_int(self) -> Int;
}

macro_rules! setup {
    ( $( $t:ty ),* ) => {
        $(
            impl const ToInt for $t {
                fn to_int(self) -> Int {
                    Int::Small(self as i128)
                }
            }
        )*
    };
}


setup!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);
