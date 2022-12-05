use crate::*;

use std::fmt::{Display, Formatter, Error};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
/// Garbage-collected wrapper around `std::string::String` implementing `Copy`.
pub struct String(pub GcCow<std::string::String>);

impl GcCompat for String {
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl GcCompat for std::string::String {
    fn points_to(&self, _m: &mut HashSet<usize>) {}
    fn as_any(&self) -> &dyn Any { self }
}

impl Display for String {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.call_ref_unchecked(|s| write!(f, "{}", s))
    }
}

/// Wrapper around the `std::format` macro, returning `libspecr::String` instead of `std::string::String`.
pub macro format {
    ($($thing:expr),*) => {
        String(GcCow::new(
            std::format!(
                $($thing),*
            )
        ))
    },
}
