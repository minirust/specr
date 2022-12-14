use crate::int::*;

impl Int {
    pub fn is_power_of_two(self) -> bool {
        let ext = self.ext();
        if let Some(uint) = ext.to_biguint() {
            uint.count_ones() == 1
        } else { false }
    }

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

        let Some(mut n) = self.ext().to_biguint() else {
            // return 1 for negative inputs.
            // should this be an error instead?
            return Int::ONE;
        };

        // powers of two are exactly the numbers having `_.count_ones() == 1`.
        while n.count_ones() != 1 {
            n = n + 1u32;
        }

        Self::wrap(ExtInt::from(n))
    }

    pub fn abs(self) -> Int {
        if self < 0 {
            self * -1i32
        } else {
            self
        }
    }

    pub fn checked_div(self, other: Int) -> Option<Int> {
        if other == 0 { return None; }
        Some(self / other)
    }

    pub fn pow(self, other: Int) -> Int {
        fn ext_pow(x: &ExtInt, other: &ExtInt) -> ExtInt {
            use num_traits::{Zero, One};

            assert!(x != &ExtInt::zero());

            if other == &ExtInt::zero()  {
                ExtInt::one()
            } else if other == &ExtInt::one() {
                x.clone()
            } else if other % 2 == ExtInt::zero() {
                let other = other >> 1;
                let a = ext_pow(x, &other);
                &a * &a
            } else {
                let other = (other-1) >> 1;
                let a = ext_pow(x, &other);
                &a * &a * x
            }
        }

        Self::wrap(ext_pow(&self.ext(), &other.ext()))
    }

    pub fn trailing_zeros(self) -> Option<Int> {
        self.ext()
            .trailing_zeros()
            .map(|x| x.into())
    }

    pub fn div_ceil(self, other: impl Into<Int>) -> Int {
        use num_integer::Integer;

        Self::wrap(self.ext().div_ceil(&other.into().ext()))
    }

    /// Returns the unique value that is equal to `self` modulo `2^size.bits()`.
    /// If `signed == Unsigned`, the result is in the interval `0..2^size.bits()`,
    /// else it is in the interval `-2^(size.bits()-1) .. 2^(size.bits()-1)`.
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
        run_sequential(|| {
            for s in [Signed, Unsigned] {
                for bits in [16, 32, 64] {
                    let size = Size::from_bits_const(bits);
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
        });
    }
}
