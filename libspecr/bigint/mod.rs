mod ops;
mod func;

use crate::specr::gccow::*;

use num_bigint::BigInt as ExtBigInt;
use num_bigint::ToBigInt as ToExtBigInt;

#[derive(Copy, Clone, Debug)]
pub struct BigInt(pub(crate) GcCow<ExtBigInt>);

impl<T: ToExtBigInt> From<T> for BigInt {
    fn from(t: T) -> BigInt {
        BigInt(gccow_new(t.to_bigint().unwrap()))
    }
}
