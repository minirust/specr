use crate::prelude::{Signedness, BigInt, Size};
use crate::specr::env::Endianness;
use crate::specr::list::{List, list};

use num_traits::cast::ToPrimitive;

// TODO preliminary implementation:
// when minirust compiles, #[test] this code against i32::from_be and similar fns.
impl Endianness {
    /// If `signed == Signed`, the data is interpreted as two's complement.
    pub fn decode(self, signed: Signedness, bytes: List<u8>) -> BigInt {
        let mut bytes = bytes;
        if matches!(self, LittleEndian) {
            bytes.reverse();
        }

        let mut out = match signed {
            Signed => BigInt::from(bytes[BigInt::zero()] as i8),
            Unsigned => BigInt::from(bytes[BigInt::zero()] as u8),
        };

        for b in &bytes[BigInt::one()..] {
            out = (out << 8) | *b;
        }

        out
    }

    /// This can fail (return `None`) if the `int` does not fit into `size` bytes,
    /// or if it is negative and `signed == Unsigned`.
    pub fn encode(self, signed: Signedness, size: Size, int: BigInt) -> Option<List<u8>> {
        if !int.in_bounds(signed, size) {
            return None;
        }

        let is_neg = int < 0;
        let mut int = int;

        if is_neg {
            int += BigInt::from(2).pow(size.bits());
        }

        let mut bytes = list![0u8; size.bytes()];

        // range-based for loops don't yet work with BigInt.
        let mut i = BigInt::zero();
        while i < size.bytes() {
            let byte = (int >> i) % 256;
            bytes[i] = byte.0.to_u8().unwrap();

            i += 1;
        }

        if is_neg {
            bytes[BigInt::zero()] |= 0b1000_0000;
        }

        if matches!(self, LittleEndian) {
            bytes.reverse();
        }

        Some(bytes)
    }
}
