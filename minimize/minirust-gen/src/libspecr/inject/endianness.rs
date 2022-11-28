use crate::libspecr::*;

use crate::prelude::{Signedness, Size};
use num_traits::cast::ToPrimitive;

fn to_u8(b: BigInt) -> u8 {
    bigint_to_usize(b) as u8
}

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
            Signedness::Signed => BigInt::from(bytes.first().unwrap() as i8),
            Signedness::Unsigned => BigInt::from(bytes.first().unwrap() as u8),
        };

        for b in bytes.iter().skip(1) {
            out = (out << 8) | b;
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

        let mut bytes = List::new();

        // first byte.
        let j = size.bytes() - 1;
        let byte = (int >> (j * 8)) % 256;
        let mut byte = to_u8(byte);
        if is_neg {
            byte |= 0b1000_0000;
        }
        bytes.push(byte);

        // all other bytes.
        // range-based for loops don't yet work with BigInt.
        let mut j = size.bytes() - 2;
        while j >= 0 {
            let byte = (int >> (j * 8)) % 256;
            let byte = to_u8(byte);
            bytes.push(byte);

            j -= 1;
        }

        if matches!(self, LittleEndian) {
            bytes.reverse();
        }

        Some(bytes)
    }
}
