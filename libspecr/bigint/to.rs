use crate::libspecr::*;

#[const_trait]
pub trait ToBigInt {
    fn to_bigint(self) -> BigInt;
}

macro_rules! setup {
    ( $( $t:ty ),* ) => {
        $(
            impl const ToBigInt for $t {
                fn to_bigint(self) -> BigInt {
                    BigInt::Small(self as i128)
                }
            }
        )*
    };
}


setup!(u8, i8, u16, i16, u32, i32, u64, i64, usize, isize);
