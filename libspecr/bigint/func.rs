use super::*;

impl BigInt {
    pub fn zero() -> BigInt {
        BigInt::from(0)
    }

    pub fn one() -> BigInt {
        BigInt::from(1)
    }

    pub fn is_power_of_two(&self) -> bool {
        if let Some(uint) = self.0.to_biguint() {
            uint.count_ones() == 1
        } else { false }
    }

    pub fn next_power_of_two(&self) -> BigInt {
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

    pub fn abs(&self) -> BigInt {
        if self.0 < ExtBigInt::from(0) {
            self.clone() * -1i32
        } else {
            self.clone()
        }
    }

    pub fn checked_div(&self, other: BigInt) -> Option<BigInt> {
        if other.0 == ExtBigInt::from(0) { return None; }
        Some(self.clone() / other)
    }

    pub fn pow(&self, other: BigInt) -> BigInt {
        assert!(self.0 != ExtBigInt::from(0));

        if other.0 == ExtBigInt::from(0) {
            BigInt::from(1)
        } else if other.0 == ExtBigInt::from(1) {
            self.clone()
        } else if other.0.clone() % ExtBigInt::from(2) == ExtBigInt::from(0) {
            let a = self.pow(other.clone()/2);
            a.clone() * a
        } else {
            let a = self.pow((other.clone()-1)/2);
            a.clone() * a * self.clone()
        }
    }
}
