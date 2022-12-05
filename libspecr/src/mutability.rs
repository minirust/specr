use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
/// Either `Mutable` or `Immutable`.
pub enum Mutability {
    Mutable,
    Immutable,
}
pub use Mutability::*;

impl GcCompat for Mutability {
    fn points_to(&self, _m: &mut HashSet<usize>) {}
    fn as_any(&self) -> &dyn Any { self }
}
