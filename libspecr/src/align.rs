use crate::*;

/// This type is basically a copy of the `Align` type in the Rust compiler.
/// See [Align](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_target/abi/struct.Align.html).
#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug, Hash, GcCompat)]
pub struct Align { raw: Int }

impl Align {
    pub const ONE: Align = Align { raw: Int::ONE };

    /// align is rounded up to the next power of two.
    pub fn from_bytes(align: impl Into<Int>) -> Option<Align> {
        let raw = align.into();
        if raw.is_power_of_two() {
            Some(Align { raw })
        } else { None }
    }

    pub const fn from_bytes_const(align: u64) -> Option<Align> {
        if align.is_power_of_two() {
            let raw = Int::from(align);
            Some(Align { raw })
        } else { None }
    }

    pub fn from_bits(align: impl Into<Int>) -> Option<Align> {
        Align::from_bytes(align.into() / 8)
    }

    pub const fn from_bits_const(align: u64) -> Option<Align> {
        Align::from_bytes_const(align / 8)
    }

    pub fn bytes(self) -> Int {
        self.raw
    }

    /// Computes the best alignment possible for the given offset
    /// (the largest power of two that the offset is a multiple of).
    /// For an offset of `0`, it returns None.
    pub fn max_for_offset(offset: Size) -> Option<Align> {
        offset.bytes().trailing_zeros()
            .map(|trailing| {
                let bytes = Int::from(2).pow(trailing);

                // `bytes = 2 ^ trailing`, hence bytes is a power of two and this unwrap() cannot fail.
                Align::from_bytes(bytes).unwrap()
            })
    }

    /// Lower the alignment, if necessary, such that the given offset
    /// is aligned to it (the offset is a multiple of the alignment).
    pub fn restrict_for_offset(self, offset: Size) -> Align {
        Align::max_for_offset(offset)
            .map(|align| align.min(self))
            .unwrap_or(self)
    }
}

