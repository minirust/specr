use crate::*;

use crate::{Size, Signedness};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
/// Either `LittleEndian` or `BigEndian`.
pub enum Endianness {
    LittleEndian,
    BigEndian,
}
pub use Endianness::*;

fn to_u8(b: Int) -> u8 {
    int_to_usize(b) as u8
}

impl Endianness {
    /// If `signed == Signed`, the data is interpreted as two's complement.
    pub fn decode(self, signed: Signedness, bytes: List<u8>) -> Int {
        let mut bytes = bytes;
        if matches!(self, LittleEndian) {
            bytes.reverse();
        }

        let mut out = match signed {
            Signedness::Signed => Int::from(bytes.first().unwrap() as i8),
            Signedness::Unsigned => Int::from(bytes.first().unwrap() as u8),
        };

        for b in bytes.iter().skip(1) {
            out = (out << 8) | b;
        }

        out
    }

    /// This can fail (return `None`) if the `int` does not fit into `size` bytes,
    /// or if it is negative and `signed == Unsigned`.
    pub fn encode(self, signed: Signedness, size: Size, int: Int) -> Option<List<u8>> {
        if !int.in_bounds(signed, size) {
            return None;
        }

        let is_neg = int < 0;
        let mut int = int;

        if is_neg {
            int += Int::from(2).pow(size.bits());
        }

        let mut bytes = List::new();

        // first byte.
        let j = size.bytes() - 1;
        let byte = (int >> (j*8)) % 256;
        let mut byte = to_u8(byte);
        if is_neg {
            byte |= 0b1000_0000;
        }
        bytes.push(byte);

        // all other bytes.
        // range-based for loops don't yet work with Int.
        let mut j = size.bytes() - 2;
        while j >= 0 {
            let byte = (int >> (j*8)) % 256;
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

#[cfg(test)]
mod tests {
    use super::*;

    macro test_encode {
        ($ty:ty, $num:expr) => {
            let i: $ty = $num;
            #[allow(unused_comparisons)]
            let signed = match <$ty>::MIN < 0 {
                true => Signed,
                false => Unsigned,
            };
            let size = Size::from_bits(<$ty>::BITS);

            for endian in [BigEndian, LittleEndian] {
                let bytes_a = endian.encode(signed, size, Int::from(i)).unwrap();
                let bytes_b = match endian {
                    BigEndian => i.to_be_bytes(),
                    LittleEndian => i.to_le_bytes(),
                };
                assert_eq!(crate::int_to_usize(bytes_a.len()), bytes_b.len());
                assert!(bytes_a.iter().zip(bytes_b).all(|(a, b)| a == b));
            }
        }
    }

    #[test]
    fn test_endianness_encode() {
        run_sequential(|| {
            for num in [i32::MIN, i32::MIN+1, -1024, -41, -2, -1, 0, 1, 2, 42, i32::MAX-1, i32::MAX] {
                test_encode!(i32, num);
            }
            for num in [i64::MIN, i64::MIN+1, -1024, -41, -2, -1, 0, 1, 2, 42, i64::MAX-1, i64::MAX] {
                test_encode!(i64, num);
            }
            for num in [0, 1, 2, 42, u32::MAX-1, u32::MAX] {
                test_encode!(u32, num);
            }
            for num in [0, 1, 2, 42, u64::MAX-1, u64::MAX] {
                test_encode!(u64, num);
            }
        });
    }

    macro test_decode {
        ($ty:ty, $num:expr) => {
            let i: $ty = $num;

            #[allow(unused_comparisons)]
            let signed = match <$ty>::MIN < 0 {
                true => Signed,
                false => Unsigned,
            };

            for endian in [BigEndian, LittleEndian] {
                let bytes = match endian {
                    BigEndian => i.to_be_bytes(),
                    LittleEndian => i.to_le_bytes(),
                };
                let bytes: List<u8> = bytes.into_iter().collect();
                let decoded = endian.decode(signed, bytes);
                assert_eq!(decoded, Int::from(i));
            }
        }
    }

    #[test]
    fn test_endianness_decode() {
        run_sequential(|| {
            for num in [i32::MIN, i32::MIN+1, -1024, -41, -2, -1, 0, 1, 2, 42, i32::MAX-1, i32::MAX] {
                test_decode!(i32, num);
            }
            for num in [i64::MIN, i64::MIN+1, -1024, -41, -2, -1, 0, 1, 2, 42, i64::MAX-1, i64::MAX] {
                test_decode!(i64, num);
            }
            for num in [0, 1, 2, 42, u32::MAX-1, u32::MAX] {
                test_decode!(u32, num);
            }
            for num in [0, 1, 2, 42, u64::MAX-1, u64::MAX] {
                test_decode!(u64, num);
            }
        });
    }
}
