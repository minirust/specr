use std::ops::*;
use std::fmt::{Formatter, Display, Error};
use std::cmp::Ordering;

use num_bigint::BigInt as ExtBigInt;
use num_bigint::ToBigInt as ToExtBigInt;

#[derive(Clone)]
pub struct BigInt(ExtBigInt);

impl ToExtBigInt for BigInt {
    fn to_bigint(&self) -> Option<ExtBigInt> {
        Some(self.0.clone())
    }
}

fn to_bigint(t: &impl ToExtBigInt) -> BigInt {
    BigInt(t.to_bigint().unwrap())
}

impl BigInt {
    pub fn from(t: i64) -> BigInt {
        Self(t.to_bigint().unwrap())
    }
}

impl Neg for BigInt {
    type Output = Self;
    fn neg(self) -> Self { Self(-self.0) }
}

impl Display for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", &self.0)
    }
}

impl Eq for BigInt {}
impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

macro_rules! setup_bigint_ops {
    ($ty:ty) => {
        impl Add<$ty> for BigInt {
            type Output = Self;
            fn add(self, other: $ty) -> Self {
                Self(self.0 + to_bigint(&other).0)
            }
        }

        impl Sub<$ty> for BigInt {
            type Output = Self;
            fn sub(self, other: $ty) -> Self {
                Self(self.0 - to_bigint(&other).0)
            }
        }

        impl Mul<$ty> for BigInt {
            type Output = Self;
            fn mul(self, other: $ty) -> Self {
                Self(self.0 * to_bigint(&other).0)
            }
        }

        impl Div<$ty> for BigInt {
            type Output = Self;
            fn div(self, other: $ty) -> Self {
                Self(self.0 / to_bigint(&other).0)
            }
        }

        impl Rem<$ty> for BigInt {
            type Output = Self;
            fn rem(self, other: $ty) -> Self {
                Self(self.0 - to_bigint(&other).0)
            }
        }

        impl Shl<$ty> for BigInt {
            type Output = Self;
            fn shl(self, other: $ty) -> Self {
                todo!()
            }
        }

        impl Shr<$ty> for BigInt {
            type Output = Self;
            fn shr(self, other: $ty) -> Self {
                todo!()
            }
        }

        impl ShlAssign<$ty> for BigInt {
            fn shl_assign(&mut self, other: $ty) {
                todo!()
            }
        }

        impl ShrAssign<$ty> for BigInt {
            fn shr_assign(&mut self, other: $ty) {
                todo!()
            }
        }

        impl BitAnd<$ty> for BigInt {
            type Output = Self;
            fn bitand(self, other: $ty) -> Self {
                Self(self.0 & to_bigint(&other).0)
            }
        }


        impl PartialOrd<$ty> for BigInt {
            fn partial_cmp(&self, other: &$ty) -> Option<Ordering> {
                self.0.partial_cmp(&to_bigint(other).0)
            }
        }

        impl PartialEq<$ty> for BigInt {
            fn eq(&self, other: &$ty) -> bool {
                self == &to_bigint(other)
            }
        }
    };
}

setup_bigint_ops!(usize);
setup_bigint_ops!(BigInt);
