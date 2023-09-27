use crate::int::*;

/// Conversion to `Int`.
///
/// This is implemented for primitive integer types and usable in `const`-contexts.
pub trait ToInt {
    /// Converts `self` to `Int`.
    fn to_int(self) -> Int;
}

macro_rules! setup {
    ( $( $t:ty ),* ) => {
        $(
            impl ToInt for $t {
                fn to_int(self) -> Int {
                    Int(IntInner::Small(self.try_into().unwrap()))
                }
            }
        )*
    };
}


setup!(u8, i8, u16, i16, u32, i32, u64, i64, i128, usize, isize);

// u128 doesn't fit into i128, hence heap alloc required.
impl ToInt for u128 {
    fn to_int(self) -> Int {
        Int::wrap(self.into())
    }
}
