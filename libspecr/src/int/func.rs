use crate::int::*;

impl Int {
    /// Returns true if `self` is a power of two. `false` otherwise.
    pub fn is_power_of_two(self) -> bool {
        let bigint = self.into_inner();
        if let Some(uint) = bigint.to_biguint() {
            uint.count_ones() == 1
        } else { false }
    }

    /// Returns the smallest power of two greater than or equal to self.
    pub fn next_power_of_two(self) -> Int {
        // faster implementation idea:
        //
        // if self <= 0: return 1
        // if self is power of two: return self
        //
        // otherwise:
        // look for most-significant one-bit,
        // and set the next significant bit to 1 instead.
        // [01010]
        //   | most-significant one!
        //
        // [10000] <- correct result

        let Some(mut n) = self.into_inner().to_biguint() else {
            // return 1 for negative inputs.
            // should this be an error instead?
            return Int::ONE;
        };

        // powers of two are exactly the numbers having `_.count_ones() == 1`.
        while n.count_ones() != 1 {
            n = n + 1u32;
        }

        Self::wrap(BigInt::from(n))
    }

    /// Computes the absolute value of self.
    pub fn abs(self) -> Int {
        if self < 0 {
            self * -1i32
        } else {
            self
        }
    }

    /// Checked integer division.
    /// Returns `None` if and only if `other == 0`.
    pub fn checked_div(self, other: Int) -> Option<Int> {
        if other == 0 { return None; }
        Some(self / other)
    }

    /// Raises `self` to the power of `other`.
    pub fn pow(self, other: Int) -> Int {
        let val = other.into_inner().to_u32().unwrap();
        Self::wrap(self.into_inner().pow(val))
    }

    /// Returns the number of least-significant bits that are zero
    /// or None if the entire number is zero.
    pub fn trailing_zeros(self) -> Option<Int> {
        self.into_inner()
            .trailing_zeros()
            .map(|x| x.into())
    }

    /// Divides `self` by `other` and rounds up the result.
    pub fn div_ceil(self, other: impl Into<Int>) -> Int {
        use num_integer::Integer;

        Self::wrap(self.into_inner().div_ceil(&other.into().into_inner()))
    }

    /// Returns the unique value that is equal to `self` modulo `2^size.bits()`.
    /// If `signed == Unsigned` the result is in the interval `0..2^size.bits()`.
    /// Otherwise it is in the interval `-2^(size.bits()-1) .. 2^(size.bits()-1)`.
    ///
    /// `size` must not be zero.
    pub fn modulo(self, signed: Signedness, size: Size) -> Int {
        if size.is_zero() {
            panic!("Int::modulo received invalid size zero!");
        }

        // the modulus.
        let m = Int::from(2).pow(size.bits());

        // n is in range `-(m-1)..m`.
        let n = self % m;

        match signed {
            // if `Unsigned`, output needs to be in range `0..m`:
            Unsigned if n < 0 => n + m,
            // if `Signed`, output needs to be in range `-m/2 .. m/2`:
            Signed if n >= m/2 => n - m,
            Signed if n < -m/2 => n + m,
            _ => n,
        }
    }

    /// Tests whether an integer is in-bounds of a finite integer type.
    pub fn in_bounds(self, signed: Signedness, size: Size) -> bool {
        self == self.modulo(signed, size)
    }

    #[doc(hidden)]
    pub fn try_to_usize(self) -> Option<usize> {
        self.into_inner().to_usize()
    }

    #[doc(hidden)]
    pub fn try_to_u8(self) -> Option<u8> {
        self.into_inner().to_u8()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // See definition of `m` in `Int::modulo`.
    fn in_bounds_helper(int: Int, signed: Signedness, size: Size) -> bool {
        let m = Int::from(2).pow(size.bits());

        let range = match signed {
            Signed => -m/2..m/2,
            Unsigned => Int::ZERO..m,
        };

        range.contains(&int)
    }

    fn test_modulo_helper(x: Int, signed: Signedness, size: Size) {
        // check in bounds
        let out = x.modulo(signed, size);
        assert!(in_bounds_helper(out, signed, size));

        // check `out == x (mod size.bits())`
        let delta = (out - x).abs();
        assert_eq!(delta % size.bits(), 0);
    }

    #[test]
    fn test_modulo() {
        for s in [Signed, Unsigned] {
            for bits in [16, 32, 64] {
                let size = Size::from_bits_const(bits).unwrap();
                let m = Int::from(2).pow(Int::from(bits));

                for base in [-m*2, -m, Int::ZERO, m, m*2] {
                    for offset1 in [-m/2, Int::ZERO, m/2] {
                        for offset2 in [-3, -2, -1, 0, 1, 2, 3] {
                            let x = base + offset1 + offset2;
                            test_modulo_helper(x, s, size);
                        }
                    }
                }
            }
        }
    }
}
