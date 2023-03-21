use crate::*;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug, PartialOrd, Ord, GcCompat)]
/// Wrapper-type for names of any kind.
pub struct Name(pub(crate) u32);

impl Name {
    // hidden so that the internal type (u32) does not leak the docs.
    #[doc(hidden)]
    pub fn from_internal(i: u32) -> Self { Name(i) }

    #[doc(hidden)]
    pub fn get_internal(self) -> u32 { self.0 }
}
