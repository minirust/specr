use crate::libspecr::*;

use std::ops::*;
use std::fmt::{Formatter, Display, Error};
use std::cmp::Ordering;

impl Display for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.ext())
    }
}

// Arithmetics
impl Neg for BigInt {
    type Output = Self;
    fn neg(self) -> Self {
        Self::wrap(-self.ext())
    }
}

impl<T: Into<BigInt>> Add<T> for BigInt {
    type Output = Self;
    fn add(self, other: T) -> Self {
        Self::wrap(self.ext() + other.into().ext())
    }
}

impl<T: Into<BigInt>> AddAssign<T> for BigInt {
    fn add_assign(&mut self, other: T) {
        *self = *self + other;
    }
}

impl<T: Into<BigInt>> Sub<T> for BigInt {
    type Output = Self;
    fn sub(self, other: T) -> Self {
        Self::wrap(self.ext() - other.into().ext())
    }
}

impl<T: Into<BigInt>> SubAssign<T> for BigInt {
    fn sub_assign(&mut self, other: T) {
        *self = *self - other;
    }
}

impl<T: Into<BigInt>> Mul<T> for BigInt {
    type Output = Self;
    fn mul(self, other: T) -> Self {
        Self::wrap(self.ext() * other.into().ext())
    }
}

impl<T: Into<BigInt>> MulAssign<T> for BigInt {
    fn mul_assign(&mut self, other: T) {
        *self = *self * other;
    }
}

impl<T: Into<BigInt>> Div<T> for BigInt {
    type Output = Self;
    fn div(self, other: T) -> Self {
        Self::wrap(self.ext() / other.into().ext())
    }
}

impl<T: Into<BigInt>> DivAssign<T> for BigInt {
    fn div_assign(&mut self, other: T) {
        *self /= *self * other;
    }
}

impl<T: Into<BigInt>> Rem<T> for BigInt {
    type Output = Self;
    fn rem(self, other: T) -> Self {
        Self::wrap(self.ext() % other.into().ext())
    }
}

impl<T: Into<BigInt>> RemAssign<T> for BigInt {
    fn rem_assign(&mut self, other: T) {
        *self %= *self * other;
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
        Self::wrap(self.ext() & other.into().ext())
    }
}

impl<T: Into<BigInt>> BitOr<T> for BigInt {
    type Output = Self;
    fn bitor(self, other: T) -> Self {
        Self::wrap(self.ext() | other.into().ext())
    }
}

// Ord
impl<T: Into<BigInt> + Clone> PartialOrd<T> for BigInt {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.ext().partial_cmp(&other.clone().into().ext())
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ext().cmp(&other.ext())
    }
}

// Eq
impl<T: Into<BigInt> + Clone> PartialEq<T> for BigInt {
    fn eq(&self, other: &T) -> bool {
        let other: BigInt = other.clone().into();
        self.ext() == other.ext()
    }
}

impl Eq for BigInt {}
