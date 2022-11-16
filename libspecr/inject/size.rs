use crate::libspecr::*;

use std::ops::*;
use std::cmp::Ordering;

use crate::prelude::Size;

impl Size {
    pub fn is_zero(&self) -> bool {
        self.bytes() == 0
    }
}

impl Add for Size {
    type Output = Size;
    fn add(self, rhs: Size) -> Size {
        let b = self.bytes() + rhs.bytes();
        Size::from_bytes(b)
    }
}

impl Mul<BigInt> for Size {
    type Output = Size;
    fn mul(self, rhs: BigInt) -> Size {
        let b = self.bytes() * rhs;
        Size::from_bytes(b)
    }
}

impl Mul<Size> for BigInt {
    type Output = Size;
    fn mul(self, rhs: Size) -> Size {
        let b = self * rhs.bytes();
        Size::from_bytes(b)
    }
}

impl PartialEq for Size {
    fn eq(&self, rhs: &Size) -> bool {
        self.bytes() == rhs.bytes()
    }
}

impl PartialOrd for Size {
    fn partial_cmp(&self, rhs: &Size) -> Option<Ordering> {
        self.bytes().partial_cmp(&rhs.bytes())
    }
}

impl Eq for Size {}
impl Ord for Size {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bytes().cmp(&other.bytes())
    }
}
