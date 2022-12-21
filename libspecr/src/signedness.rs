use crate::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, GcCompat)]
/// Either `Signed` or `Unsigned`.
pub enum Signedness {
    Signed,
    Unsigned
}
pub use Signedness::*;
