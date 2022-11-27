use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum Signedness {
    Signed,
    Unsigned
}

pub use Signedness::*;

impl GcCompat for Signedness {
    fn points_to(&self, m: &mut HashSet<usize>) {}
    fn as_any(&self) -> &dyn Any { self }
}
