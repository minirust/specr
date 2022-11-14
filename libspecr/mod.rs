use std::collections::{HashSet, HashMap};

// contains the BigInt implemenation.
// accessed using `BigInt`.
mod bigint;
pub use bigint::BigInt;

mod list;

// contains hidden functions that are called only due to generated code.
// accessed using `specr::hidden::_`.
pub mod hidden;

// contains items to be exposed to the user without importing.
// like List, Set etc.
// accessed using `_`.
#[macro_use]
mod env;
pub use env::*;

// contains implementation for opaque items from MiniRust, which are not already defined in mirror.
// accessed using `specr::_`.
#[macro_use]
mod intrinsics;
pub use intrinsics::*;

// implements some operators for Size.
mod impls;

// implements a small clone-on-write garbage collector.
mod gccow;

pub mod prelude {
    pub use crate::specr::BigInt;
    pub use crate::specr::env::*;
    pub use crate::specr::list::*;
    pub use std::hash::Hash;
    pub use std::fmt::Debug;
}
