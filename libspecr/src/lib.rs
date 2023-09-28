//! `libspecr` is the standard library of specr lang.

#![warn(missing_docs)]

// We need nightly features since we want to imbue `NdResult` with `?` behavior,
// and we want to be able to call `Result`-returning functions from inside
// `NdResult` seamlessly.
#![feature(try_trait_v2)]
#![feature(try_trait_v2_yeet)]
#![feature(try_trait_v2_residual)]
#![feature(decl_macro)]
#![feature(iterator_try_collect)]
#![feature(step_trait)]

use std::collections::HashSet;
use std::any::Any;
use std::fmt::Debug;
use std::hash::Hash;

use im::HashSet as IMHashSet;
use im::HashMap as IMHashMap;
use im::Vector as IMVector;

// Re-export the derive macro so reverse dependencies can use it
// without directly depending on gccompat_derive.
pub use gccompat_derive::GcCompat;

mod int;
pub use int::*;

mod option;

mod ndresult;
pub use ndresult::*;

mod ret;
pub use ret::*;

mod size;
pub use size::*;

mod align;
pub use align::*;

mod list;
pub use list::*;

mod set;
pub use set::*;

mod map;
pub use map::*;

mod write;
pub use write::*;

mod string;
pub use string::*;

mod name;
pub use name::*;

mod nondet;
pub use nondet::*;

mod distribution;
pub use distribution::*;

mod endianness;

mod mutability;

mod signedness;
use signedness::*;

mod gc;
use gc::*;

mod obj;
use obj::*;

#[doc(hidden)]
pub mod hidden {
    pub use crate::obj::*;
    pub use crate::gc::{GcCow, GcCompat, mark_and_sweep, clear};
}

pub use crate::nondet::*;
pub use crate::name::*;

/// The items from this module are automatically imported.
pub mod prelude {
    pub use crate::ret;
    pub use crate::Align;
    pub use crate::Size;
    pub use crate::Int;
    pub use crate::list::*;
    pub use crate::set::*;
    pub use crate::map::*;
    pub use crate::write::*;
    pub use crate::endianness::*;
    pub use crate::mutability::*;
    pub use crate::signedness::*;
    pub use crate::string::{String, format};
    pub use crate::nondet::{pick, predict};
    pub use crate::option::*;

    pub use std::hash::Hash;
    pub use std::fmt::Debug;
}

// This exists so that `gccompat-derive` can use `libspecr::hidden::GcCompat` to address GcCompat,
// whether it's used within libspecr or not.
pub(crate) use crate as libspecr;
