use crate::*;

use std::ops::*;
use std::fmt::{Formatter, Display, Error};
use std::cmp::Ordering;
use num_traits::ToPrimitive;

impl Display for Int {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}", self.ext())
    }
}

// Arithmetics
impl Neg for Int {
    type Output = Self;
    fn neg(self) -> Self {
        Self::wrap(-self.ext())
    }
}

impl<T: Into<Int>> Add<T> for Int {
    type Output = Self;
    fn add(self, other: T) -> Self {
        Self::wrap(self.ext() + other.into().ext())
    }
}

impl<T: Into<Int>> AddAssign<T> for Int {
    fn add_assign(&mut self, other: T) {
        *self = *self + other;
    }
}

impl<T: Into<Int>> Sub<T> for Int {
    type Output = Self;
    fn sub(self, other: T) -> Self {
        Self::wrap(self.ext() - other.into().ext())
    }
}

impl<T: Into<Int>> SubAssign<T> for Int {
    fn sub_assign(&mut self, other: T) {
        *self = *self - other;
    }
}

impl<T: Into<Int>> Mul<T> for Int {
    type Output = Self;
    fn mul(self, other: T) -> Self {
        Self::wrap(self.ext() * other.into().ext())
    }
}

impl<T: Into<Int>> MulAssign<T> for Int {
    fn mul_assign(&mut self, other: T) {
        *self = *self * other;
    }
}

impl<T: Into<Int>> Div<T> for Int {
    type Output = Self;
    fn div(self, other: T) -> Self {
        Self::wrap(self.ext() / other.into().ext())
    }
}

impl<T: Into<Int>> DivAssign<T> for Int {
    fn div_assign(&mut self, other: T) {
        *self = *self / other;
    }
}

impl<T: Into<Int>> Rem<T> for Int {
    type Output = Self;
    fn rem(self, other: T) -> Self {
        Self::wrap(self.ext() % other.into().ext())
    }
}

impl<T: Into<Int>> RemAssign<T> for Int {
    fn rem_assign(&mut self, other: T) {
        *self = *self % other;
    }
}

impl<T: Into<Int>> Shl<T> for Int {
    type Output = Self;
    fn shl(self, other: T) -> Self {
        if self == 0 { return self; }

        let i = other.into().ext().to_i128().unwrap();
        Self::wrap(self.ext() << i)
    }
}


impl<T: Into<Int>> ShlAssign<T> for Int {
    fn shl_assign(&mut self, other: T) {
        *self = *self << other;
    }
}

impl<T: Into<Int>> Shr<T> for Int {
    type Output = Self;
    fn shr(self, other: T) -> Self {
        if self == 0 { return self; }

        let i = other.into().ext().to_i128().unwrap();
        Self::wrap(self.ext() >> i)
    }
}

impl<T: Into<Int>> ShrAssign<T> for Int {
    fn shr_assign(&mut self, other: T) {
        *self = *self >> other;
    }
}

impl<T: Into<Int>> BitAnd<T> for Int {
    type Output = Self;
    fn bitand(self, other: T) -> Self {
        Self::wrap(self.ext() & other.into().ext())
    }
}

impl<T: Into<Int>> BitOr<T> for Int {
    type Output = Self;
    fn bitor(self, other: T) -> Self {
        Self::wrap(self.ext() | other.into().ext())
    }
}

// Ord
impl<T: Into<Int> + Clone> PartialOrd<T> for Int {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.ext().partial_cmp(&other.clone().into().ext())
    }
}

impl Ord for Int {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ext().cmp(&other.ext())
    }
}

// Eq
impl<T: Into<Int> + Clone> PartialEq<T> for Int {
    fn eq(&self, other: &T) -> bool {
        let other: Int = other.clone().into();
        self.ext() == other.ext()
    }
}

impl Eq for Int {}
