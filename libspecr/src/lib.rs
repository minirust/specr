#![feature(let_else)]
#![feature(try_trait_v2)]
#![feature(try_trait_v2_yeet)]
#![feature(try_trait_v2_residual)]
#![feature(yeet_expr)]

use std::collections::{HashSet, HashMap};

// contains the BigInt implemenation.
// accessed using `BigInt`.
mod bigint;
pub use bigint::BigInt;

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

pub mod prelude {
    pub use crate::BigInt;
    pub use crate::env::*;
}
