mod func;
mod wrapper;

use crate::*;

use self::wrapper::WriteWrapper;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, GcCompat)]
/// Garbage-collected datastructure representing a write stream and implementing `Copy`.
pub struct DynWrite(pub(crate) GcCow<WriteWrapper>);

