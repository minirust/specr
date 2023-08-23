use crate::*;

/// This type is basically a copy of the `Align` type in the Rust compiler.
/// See [Align](https://doc.rust-lang.org/nightly/nightly-rustc/rustc_target/abi/struct.Align.html).
#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Debug, Hash, GcCompat)]
pub struct Align { raw: Int }

impl Align {
    /// The `1 byte` alignment.
    pub const ONE: Align = Align { raw: Int::ONE };

    /// Constructs `Align` with `align` many bytes.
    /// Returns `None` if `align` is not a power of two.
    pub fn from_bytes(align: impl Into<Int>) -> Option<Align> {
        let raw = align.into();
        if raw.is_power_of_two() {
            Some(Align { raw })
        } else { None }
    }

    /// Variation of `from_bytes` for const contexts.
    pub const fn from_bytes_const(align: u64) -> Option<Align> {
        if align.is_power_of_two() {
            let raw = Int::const_from(align);
            Some(Align { raw })
        } else { None }
    }

    /// Constructs `Align` with `align` many bits.
    /// Returns `None` if `align` is not divisible by 8, or if `align/8` is no power of two.
    pub fn from_bits(align: impl Into<Int>) -> Option<Align> {
        let align = align.into();
        if align % 8 != 0 { return None; }
        Align::from_bytes(align / 8)
    }

    /// Variation of `from_bits` for const contexts.
    pub const fn from_bits_const(align: u64) -> Option<Align> {
        if align % 8 != 0 { return None; }
        Align::from_bytes_const(align / 8)
    }

    /// The number of bytes of `self`.
    pub fn bytes(self) -> Int {
        self.raw
    }

    /// Computes the best alignment possible for the given offset
    /// (the largest power of two that the offset is a multiple of).
    /// For an offset of `0` it returns None.
    pub fn max_for_offset(offset: Size) -> Option<Align> {
        offset.bytes().trailing_zeros()
            .map(|trailing| {
                let bytes = Int::from(2).pow(trailing);

                // `bytes = 2 ^ trailing`, hence bytes is a power of two and this unwrap() cannot fail.
                Align::from_bytes(bytes).unwrap()
            })
    }

    /// Lower the alignment if necessary, such that the given offset
    /// is aligned to it (the offset is a multiple of the alignment).
    pub fn restrict_for_offset(self, offset: Size) -> Align {
        Align::max_for_offset(offset)
            .map(|align| align.min(self))
            .unwrap_or(self)
    }

    /// Check if the given address is sufficiently aligned.
    pub fn is_aligned(self, addr: Int) -> bool {
        addr % self.bytes() == 0
    }
}

