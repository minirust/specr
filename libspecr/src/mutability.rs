use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
/// Either `Mutable` or `Immutable`.
#[derive(GcCompat)]
pub enum Mutability {
    Mutable,
    Immutable,
}
pub use Mutability::*;
