use std::ops::*;

use crate::baselib::Signedness;

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
        self.0 == other.to_bigint()
    }
}

impl BigInt {
	pub fn checked_div(self, other: BigInt) -> Option<BigInt> {
        self.0.checked_div().map(Self)
	}

	pub fn modulo(self, _: Signedness, other: usize) -> BigInt {
        todo!()
	}
}

