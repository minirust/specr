use crate::libspecr::*;

use std::ops::*;
use std::fmt::{Formatter, Display, Error};
use std::cmp::Ordering;

impl Display for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.call_ref(|b| write!(f, "{}", b))
    }
}

fn mk_bigint(b: ExtBigInt) -> BigInt {
    BigInt(gccow_new(b))
}

// Arithmetics
impl Neg for BigInt {
    type Output = Self;
    fn neg(self) -> Self {
        self.0.call_ref(|b| mk_bigint(-b))
    }
}

impl<T: Into<BigInt>> Add<T> for BigInt {
    type Output = Self;
    fn add(self, other: T) -> Self {
        self.0.call_ref1(other.into().0, |b, o| mk_bigint(b + o))
    }
}

impl<T: Into<BigInt>> AddAssign<T> for BigInt {
    fn add_assign(&mut self, other: T) {
        self.0.call_mut1(other.into().0, |b, o| {
            *b += o;
        });
    }
}

impl<T: Into<BigInt>> Sub<T> for BigInt {
    type Output = Self;
    fn sub(self, other: T) -> Self {
        self.0.call_ref1(other.into().0, |b, o| mk_bigint(b - o))
    }
}

impl<T: Into<BigInt>> SubAssign<T> for BigInt {
    fn sub_assign(&mut self, other: T) {
        self.0.call_mut1(other.into().0, |b, o| {
            *b -= o;
        });
    }
}

impl<T: Into<BigInt>> Mul<T> for BigInt {
    type Output = Self;
    fn mul(self, other: T) -> Self {
        self.0.call_ref1(other.into().0, |b, o| mk_bigint(b * o))
    }
}

impl<T: Into<BigInt>> MulAssign<T> for BigInt {
    fn mul_assign(&mut self, other: T) {
        self.0.call_mut1(other.into().0, |b, o| {
            *b *= o;
        });
    }
}

impl<T: Into<BigInt>> Div<T> for BigInt {
    type Output = Self;
    fn div(self, other: T) -> Self {
        self.0.call_ref1(other.into().0, |b, o| mk_bigint(b / o))
    }
}

impl<T: Into<BigInt>> DivAssign<T> for BigInt {
    fn div_assign(&mut self, other: T) {
        self.0.call_mut1(other.into().0, |b, o| {
            *b /= o;
        });
    }
}

impl<T: Into<BigInt>> Rem<T> for BigInt {
    type Output = Self;
    fn rem(self, other: T) -> Self {
        self.0.call_ref1(other.into().0, |b, o| mk_bigint(b % o))
    }
}

impl<T: Into<BigInt>> RemAssign<T> for BigInt {
    fn rem_assign(&mut self, other: T) {
        self.0.call_mut1(other.into().0, |b, o| {
            *b %= o;
        });
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
        self.0.call_ref1(other.into().0, |b, o| mk_bigint(b & o))
    }
}

impl<T: Into<BigInt>> BitOr<T> for BigInt {
    type Output = Self;
    fn bitor(self, other: T) -> Self {
        self.0.call_ref1(other.into().0, |b, o| mk_bigint(b | o))
    }
}

// Ord
impl<T: Into<BigInt> + Clone> PartialOrd<T> for BigInt {
    fn partial_cmp(&self, other: &T) -> Option<Ordering> {
        self.0.call_ref1(other.clone().into().0, |b, o| b.partial_cmp(&o))
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.call_ref1(other.0, |b, o| b.cmp(&o))
    }
}

// Eq
impl<T: Into<BigInt> + Clone> PartialEq<T> for BigInt {
    fn eq(&self, other: &T) -> bool {
        self.0.call_ref1(other.clone().into().0, |b, o| b == o)
    }
}

impl Eq for BigInt {}
