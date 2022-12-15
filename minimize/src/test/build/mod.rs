//! This module makes it easy to create a `Program`.

use crate::test::*;

mod function;
pub use function::*;

mod statement; // also includes terminators
pub use statement::*;

mod expr;
pub use expr::*;

mod ty;
pub use ty::*;

pub fn align(bytes: u32) -> Align {
    Align::from_bytes(bytes)
}

pub fn size(bytes: u32) -> Size {
    Size::from_bytes(bytes)
}
