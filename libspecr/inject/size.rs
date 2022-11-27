use crate::libspecr::*;

use std::ops::*;

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

impl Mul<Int> for Size {
    type Output = Size;
    fn mul(self, rhs: Int) -> Size {
        let b = self.bytes() * rhs;
        Size::from_bytes(b)
    }
}

impl Mul<Size> for Int {
    type Output = Size;
    fn mul(self, rhs: Size) -> Size {
        let b = self * rhs.bytes();
        Size::from_bytes(b)
    }
}
