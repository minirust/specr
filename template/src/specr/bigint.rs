use std::ops::*;

use num_bigint::ToBigInt;

pub struct BigInt(num_bigint::BigInt);

impl Neg for BigInt {
	type Output = Self;
	fn neg(self) -> Self { Self(-self.0) }
}

impl Add for BigInt {
	type Output = Self;
	fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

impl Sub for BigInt {
	type Output = Self;
	fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0)
    }
}

impl Mul for BigInt {
	type Output = Self;
	fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0)
    }
}

impl PartialEq<usize> for BigInt {
	fn eq(&self, other: &usize) -> bool {
        Some(&self.0) == other.to_bigint().as_ref()
    }
}
