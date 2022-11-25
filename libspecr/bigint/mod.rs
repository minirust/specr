use crate::libspecr::*;

mod ops;
mod func;
mod to;

pub use to::ToBigInt;

/// The external Bigint Type, which we use under the hood.
pub use num_bigint::BigInt as ExtBigInt;

#[derive(Copy, Clone, Debug, Hash)]
pub enum BigInt {
    Big(GcCow<ExtBigInt>),
    /// i128 is used to contain u64 and i64.
    Small(i128),
}

impl<T: ~const ToBigInt> const From<T> for BigInt {
    fn from(t: T) -> BigInt {
        t.to_bigint()
    }
}

impl GcCompat for BigInt {
    fn points_to(&self, m: &mut HashSet<usize>) {
        match self {
            Self::Big(x) => x.points_to(m),
            Self::Small(_) => {},
        }
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl GcCompat for ExtBigInt {
    fn points_to(&self, _m: &mut HashSet<usize>) {}
    fn as_any(&self) -> &dyn Any { self }
}

impl BigInt {
    pub(in crate::libspecr) fn ext(self) -> ExtBigInt {
        use num_bigint::ToBigInt;

        match self {
            Self::Big(x) => x.get(),
            Self::Small(x) => x.into(),
        }
    }

    pub(in crate::libspecr) fn wrap(ext: ExtBigInt) -> Self {
        Self::Big(
            GcCow::new(ext)
        )
    }
}
