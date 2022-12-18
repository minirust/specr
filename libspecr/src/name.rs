use crate::*;

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug, PartialOrd, Ord)]
/// Wrapper-type for names of any kind.
pub struct Name(pub(crate) u32);

impl GcCompat for Name {
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl Name {
    // hidden so that the internal type (u32) does not leak the docs.
    #[doc(hidden)]
    pub fn new(i: u32) -> Self { Name(i) }

    #[doc(hidden)]
    pub fn get(self) -> u32 { self.0 }
}
