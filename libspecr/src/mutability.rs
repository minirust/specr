use crate::*;
use ::serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
/// Either `Mutable` or `Immutable`.
#[derive(GcCompat)]
pub enum Mutability {
    #[allow(missing_docs)]
    Mutable,
    #[allow(missing_docs)]
    Immutable,
}
pub use Mutability::*;
