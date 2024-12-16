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

    /// If `other` is positive, calculates the smallest value greater than or equal to self that is a multiple of `other`.
    /// If `other` is negative, calculates the largest value less than or equal to self that is a multiple of `other`.
    ///
    /// Panics if `other` is zero.
    pub fn next_multiple_of(self, other: Int) -> Int {
        self.div_ceil(other) * other
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
        // We only support powers that fit into u32; this is more than enough for now.
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

    /// Calculates Euclidean division: the result is rounded towards -INF 
    /// for `other > 0` and towards +INF for `other < 0`.
    /// The result `n` satisfies `self == n * other + self.rem_euclid(other)`.
    pub fn div_euclid(self, other: Int) -> Int {
        let q = self / other;
        if self % other < 0 {
            if other > 0 { q - 1 } else { q + 1 }
        } else {
            q
        }
    }

    /// Calculate nonnegative remainder of `self (mod other)`.
    /// The return value is in the range `0..other.abs()`.
    pub fn rem_euclid(self, other: Int) -> Int {
        let rem = self % other;
        if rem < 0 { 
            rem + other.abs() 
        } else { 
            rem
        }
    }

    /// Returns the unique value that is equal to `self` modulo `2^size.bits()`
    /// and within the bounds of a finite integer type with the given signedness and size.
    /// If `signed == Unsigned` the result is in the interval `0..2^size.bits()`.
    /// Otherwise it is in the interval `-2^(size.bits()-1) .. 2^(size.bits()-1)`.
    ///
    /// `size` must not be zero.
    pub fn bring_in_bounds(self, signed: Signedness, size: Size) -> Int {
        if size.is_zero() {
            panic!("Int::modulo received invalid size zero!");
        }

        // the modulus.
        let m = Int::from(2).pow(size.bits());

        // `rem` is in range `0..m`.
        let rem = self.rem_euclid(m);

        match signed {
            Unsigned => rem, // already in the right range
            Signed =>
                // Bring value into the right range
                if rem >= m/2 {
                    rem - m
                } else {
                    rem
                }
        }
    }

    /// Tests whether an integer is in-bounds of a finite integer type.
    pub fn in_bounds(self, signed: Signedness, size: Size) -> bool {
        self == self.bring_in_bounds(signed, size)
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

    fn bring_in_bounds_helper(x: Int, signed: Signedness, size: Size) {
        // check in bounds
        let out = x.bring_in_bounds(signed, size);
        assert!(in_bounds_helper(out, signed, size));

        // check `out == x (mod size.bits())`
        let delta = (out - x).abs();
        assert_eq!(delta % size.bits(), 0);
    }

    #[test]
    fn bring_in_bounds() {
        for s in [Signed, Unsigned] {
            for bits in [16, 32, 64] {
                let size = Size::from_bits_const(bits).unwrap();
                let m = Int::from(2).pow(Int::from(bits));

                for base in [-m*2, -m, Int::ZERO, m, m*2] {
                    for offset1 in [-m/2, Int::ZERO, m/2] {
                        for offset2 in [-3, -2, -1, 0, 1, 2, 3] {
                            let x = base + offset1 + offset2;
                            bring_in_bounds_helper(x, s, size);
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn div_euclid() {
        let a = Int::from(7);
        let b = Int::from(4);
        assert_eq!(a.div_euclid(b), Int::from(1)); //  7 =  1 *  4 + 3
        assert_eq!((-a).div_euclid(b), Int::from(-2)); // -7 = -2 *  4 + 1
        assert_eq!(a.div_euclid(-b), Int::from(-1)); //  7 = -1 * -4 + 3
        assert_eq!((-a).div_euclid(-b), Int::from(2)); // -7 =  2 * -4 + 1
    }

    #[test]
    fn rem_euclid() {
        let a = Int::from(7);
        let b = Int::from(4);
        assert_eq!(a.rem_euclid(b), Int::from(3));
        assert_eq!((-a).rem_euclid(b), Int::from(1));
        assert_eq!(a.rem_euclid(-b), Int::from(3));
        assert_eq!((-a).rem_euclid(-b), Int::from(1));
    }

    /// Test cases from <https://doc.rust-lang.org/nightly/std/primitive.i32.html#method.next_multiple_of>
    #[test]
    fn next_multiple_of() {
        assert_eq!(Int::from(16).next_multiple_of(Int::from(8)), Int::from(16));
        assert_eq!(Int::from(23).next_multiple_of(Int::from(8)), Int::from(24));
        assert_eq!(Int::from(16).next_multiple_of(Int::from(-8)), Int::from(16));
        assert_eq!(Int::from(23).next_multiple_of(Int::from(-8)), Int::from(16));
        assert_eq!(
            Int::from(-16_i32).next_multiple_of(Int::from(8)),
            Int::from(-16)
        );
        assert_eq!(
            Int::from(-23_i32).next_multiple_of(Int::from(8)),
            Int::from(-16)
        );
        assert_eq!(
            Int::from(-16_i32).next_multiple_of(Int::from(-8)),
            Int::from(-16)
        );
        assert_eq!(
            Int::from(-23_i32).next_multiple_of(Int::from(-8)),
            Int::from(-24)
        );
    }
}
