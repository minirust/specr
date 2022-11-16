use crate::libspecr::*;

mod ops;
mod func;

pub use num_bigint::BigInt as ExtBigInt;
pub use num_bigint::ToBigInt as ToExtBigInt;

#[derive(Copy, Clone, Debug)]
pub struct BigInt(pub GcCow<ExtBigInt>);

impl<T: ToExtBigInt> From<T> for BigInt {
    fn from(t: T) -> BigInt {
        BigInt(gccow_new(t.to_bigint().unwrap()))
    }
}

impl GcCompat for BigInt {
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl GcCompat for ExtBigInt {
    fn points_to(&self, _m: &mut HashSet<usize>) {}
    fn as_any(&self) -> &dyn Any { self }
}
