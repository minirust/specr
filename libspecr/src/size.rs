use std::fmt;
use std::ops::{Add, Mul};

use crate::*;

/// `Size` represents a non-negative number of bytes or bits.
///
/// It is basically a copy of the `Size` type in the Rust compiler.
/// See [Size](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_target/abi/struct.Size.html).
///
/// Note that the `Size` type has no upper-bound.
/// Users needs check whether a given `Size` is too large for their Machine themselves.
#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash, GcCompat)]
pub struct Size { raw: Int }

// This is debug-printed a lot in larger structs, don't waste too much space there
impl fmt::Debug for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Size({} bytes)", self.bytes())
    }
}

impl Size {
    /// The "0 bytes" size.
    pub const ZERO: Size = Size { raw: Int::ZERO };

    /// Returns None if `bits` is negative or not divisible by 8.
    pub fn from_bits(bits: impl Into<Int>) -> Option<Size> {
        let bits = bits.into();
        if bits % 8 != 0 { return None; };
        if bits < 0 { return None; }

        let raw = bits / 8;
        Some(Size { raw })
    }

    /// Variation of `from_bits` for const contexts.
    /// Returns None if `bits` is not divisible by 8.
    pub const fn from_bits_const(bits: u64) -> Option<Size> {
        if bits % 8 != 0 { return None; }
        let bytes = bits / 8;
        let raw = Int::const_from(bytes);
        Some(Size { raw })
    }

    /// Returns None if `bytes` is negative.
    pub fn from_bytes(bytes: impl Into<Int>) -> Option<Size> {
        let bytes = bytes.into();
        if bytes < 0 { return None; }

        Some(Size { raw: bytes })
    }

    /// Variation of `from_bytes` for const contexts.
    /// Cannot fail since the input is unsigned and already in bytes.
    pub const fn from_bytes_const(bytes: u64) -> Size {
        let raw = Int::const_from(bytes);
        Size { raw }
    }

    /// The number of bytes of `self`
    pub fn bytes(self) -> Int { self.raw }

    /// The number of bits of `self`
    pub fn bits(self) -> Int { self.raw * 8 }

    /// Returns whether `self` is `Size::ZERO`
    pub fn is_zero(&self) -> bool {
        self.bytes() == 0
    }
}

impl Add for Size {
    type Output = Size;
    fn add(self, rhs: Size) -> Size {
        let b = self.bytes() + rhs.bytes();
        Size::from_bytes(b).unwrap()
    }
}

impl Mul<Int> for Size {
    type Output = Size;
    fn mul(self, rhs: Int) -> Size {
        let b = self.bytes() * rhs;
        Size::from_bytes(b).unwrap()
    }
}

impl Mul<Size> for Int {
    type Output = Size;
    fn mul(self, rhs: Size) -> Size {
        let b = self * rhs.bytes();
        Size::from_bytes(b).unwrap()
    }
}

