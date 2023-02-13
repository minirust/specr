use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
/// Either `Mutable` or `Immutable`.
#[derive(GcCompat)]
pub enum Mutability {
    #[allow(missing_docs)]
    Mutable,
    #[allow(missing_docs)]
    Immutable,
}
pub use Mutability::*;
