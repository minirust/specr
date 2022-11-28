use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
/// Either `Signed` or `Unsigned`.
pub enum Signedness {
    Signed,
    Unsigned
}

impl GcCompat for Signedness {
    fn points_to(&self, _m: &mut HashSet<usize>) {}
    fn as_any(&self) -> &dyn Any { self }
}
