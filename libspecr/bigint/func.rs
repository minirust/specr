use crate::libspecr::bigint::*;

impl BigInt {
    pub fn zero() -> BigInt {
        BigInt::from(0)
    }

    pub fn one() -> BigInt {
        BigInt::from(1)
    }

    pub fn is_power_of_two(self) -> bool {
        self.0.call_ref(|b| {
            if let Some(uint) = b.to_biguint() {
                uint.count_ones() == 1
            } else { false }
        })
    }

    pub fn next_power_of_two(self) -> BigInt {
        // TODO improve implementation

        // better implementation idea:
        // return self, is already power of two.
        // if self == 0, return 1.
        // otherwise:
        // look for most-significant one-bit,
        // and set the next significant bit to 1 instead.
        // [01010]
        //   | most-significant one!
        //
        // [10000] <- correct result

        let mut n = self.clone();
        while !n.is_power_of_two() {
            n = n + 1;
        }

        n
    }

    pub fn abs(self) -> BigInt {
        if self < 0 {
            self * -1i32
        } else {
            self
        }
    }

    pub fn checked_div(self, other: BigInt) -> Option<BigInt> {
        if other == 0 { return None; }
        Some(self / other)
    }

    pub fn pow(self, other: BigInt) -> BigInt {
        assert!(self != 0);

        if other == 0 {
            BigInt::one()
        } else if other == 1 {
            self
        } else if other % 2 == 0 {
            let a = self.pow(other/2);
            a * a
        } else {
            let a = self.pow((other-1)/2);
            a * a * self
        }
    }
}
