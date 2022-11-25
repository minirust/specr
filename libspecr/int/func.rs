use crate::libspecr::int::*;

impl Int {
    pub const ZERO: Int = Int::Small(0);
    pub const ONE: Int = Int::Small(1);

    // TODO deprecate
    pub const fn zero() -> Int { Self::ZERO }
    pub const fn one() -> Int { Self::ONE }

    pub fn is_power_of_two(self) -> bool {
        let ext = self.ext();
        if let Some(uint) = ext.to_biguint() {
            uint.count_ones() == 1
        } else { false }
    }

    pub fn next_power_of_two(self) -> Int {
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
        assert!(self != 0);

        if other == 0 {
            Int::one()
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

    pub fn trailing_zeros(self) -> Option<Int> {
        self.ext()
            .trailing_zeros()
            .map(|x| x.into())
    }
}
