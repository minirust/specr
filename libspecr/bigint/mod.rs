mod ops;
mod func;

use num_bigint::BigInt as ExtBigInt;
use num_bigint::ToBigInt as ToExtBigInt;

#[derive(Clone, Debug)]
pub struct BigInt(pub(crate) ExtBigInt);

impl<T: ToExtBigInt> From<T> for BigInt {
    fn from(t: T) -> BigInt {
        BigInt(t.to_bigint().unwrap())
    }
}
