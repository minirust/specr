use crate::*;
use ::serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, GcCompat, Serialize, Deserialize)]
/// Expresses whether an integer has a sign or not
pub enum Signedness {
    #[allow(missing_docs)]
    Signed,
    #[allow(missing_docs)]
    Unsigned
}
pub use Signedness::*;
