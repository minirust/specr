use crate::libspecr::*;

mod ops;
mod func;
mod to;

pub use to::ToInt;

/// The external Bigint Type, which we use under the hood.
pub use num_bigint::BigInt as ExtInt;

#[derive(Copy, Clone, Debug, Hash)]
pub enum Int {
    Big(GcCow<ExtInt>),
    /// i128 is used to contain u64 and i64.
    Small(i128),
}

impl<T: ~const ToInt> const From<T> for Int {
    fn from(t: T) -> Int {
        t.to_int()
    }
}

impl GcCompat for Int {
    fn points_to(&self, m: &mut HashSet<usize>) {
        match self {
            Self::Big(x) => x.points_to(m),
            Self::Small(_) => {},
        }
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl GcCompat for ExtInt {
    fn points_to(&self, _m: &mut HashSet<usize>) {}
    fn as_any(&self) -> &dyn Any { self }
}

impl Int {
    pub(in crate::libspecr) fn ext(self) -> ExtInt {
        use num_bigint::ToBigInt;

        match self {
            Self::Big(x) => x.get(),
            Self::Small(x) => x.into(),
        }
    }

    pub(in crate::libspecr) fn wrap(ext: ExtInt) -> Self {
        Self::Big(
            GcCow::new(ext)
        )
    }
}
