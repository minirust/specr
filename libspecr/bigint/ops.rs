use super::*;

use std::ops::*;
use std::fmt::{Formatter, Display, Error};
use std::cmp::Ordering;

impl Display for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", &self.0)
    }
}

// Arithmetics
impl Neg for BigInt {
    type Output = Self;
    fn neg(self) -> Self { Self(-self.0) }
}

impl<T: Into<BigInt>> Add<T> for BigInt {
    type Output = Self;
    fn add(self, other: T) -> Self {
        Self(self.0 + other.into().0)
    }
}

impl<T: Into<BigInt>> AddAssign<T> for BigInt {
    fn add_assign(&mut self, other: T) {
        self.0 += other.into().0;
    }
}

impl<T: Into<BigInt>> Sub<T> for BigInt {
    type Output = Self;
    fn sub(self, other: T) -> Self {
        Self(self.0 - other.into().0)
    }
}

impl<T: Into<BigInt>> Mul<T> for BigInt {
    type Output = Self;
    fn mul(self, other: T) -> Self {
        Self(self.0 * other.into().0)
    }
}

impl<T: Into<BigInt>> Div<T> for BigInt {
    type Output = Self;
    fn div(self, other: T) -> Self {
        Self(self.0 / other.into().0)
    }
}

impl<T: Into<BigInt>> Rem<T> for BigInt {
    type Output = Self;
    fn rem(self, other: T) -> Self {
        Self(self.0 - other.into().0)
    }
}

impl<T: Into<BigInt>> Shl<T> for BigInt {
    type Output = Self;
    fn shl(self, _other: T) -> Self {
        todo!()
    }
}


impl<T: Into<BigInt>> ShlAssign<T> for BigInt {
    fn shl_assign(&mut self, _other: T) {
        todo!()
    }
}

impl<T: Into<BigInt>> Shr<T> for BigInt {
    type Output = Self;
    fn shr(self, _other: T) -> Self {
        todo!()
    }
}

impl<T: Into<BigInt>> ShrAssign<T> for BigInt {
    fn shr_assign(&mut self, _other: T) {
        todo!()
    }
}

impl<T: Into<BigInt>> BitAnd<T> for BigInt {
    type Output = Self;
    fn bitand(self, other: T) -> Self {
        Self(self.0 & other.into().0)
    }
}

impl<T: Into<BigInt>> BitOr<T> for BigInt {
    type Output = Self;
    fn bitor(self, other: T) -> Self {
        Self(self.0 | other.into().0)
    }
}

// Ord
impl<T: Into<BigInt> + Clone> PartialOrd<T> for BigInt {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        let other: BigInt = other.clone().into();
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

// Eq
impl<T: Into<BigInt> + Clone> PartialEq<T> for BigInt {
    fn eq(&self, other: &T) -> bool {
        let other: BigInt = other.clone().into();
        self == &other
    }
}

impl Eq for BigInt {}
