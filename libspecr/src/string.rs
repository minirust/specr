use crate::*;

use std::fmt::{Display, Formatter, Error};

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash, GcCompat)]
/// Garbage-collected wrapper around `std::string::String` implementing `Copy`.
pub struct String(pub(crate) GcCow<std::string::String>);

impl Display for String {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.call_ref_unchecked(|s| write!(f, "{}", s))
    }
}

impl String {
    #[doc(hidden)]
    pub fn from_internal(s: std::string::String) -> Self {
        Self(GcCow::new(s))
    }

    #[doc(hidden)]
    pub fn get_internal(self) -> std::string::String {
        self.0.extract()
    }
}

/// Wrapper around the `std::format` macro returning `libspecr::String` instead of `std::string::String`.
pub macro format {
    ($($thing:expr),*) => {
        String::from_internal(
            std::format!(
                $($thing),*
            )
        )
    },
}
