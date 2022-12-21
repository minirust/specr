#![feature(int_roundings)]
#![feature(const_trait_impl)]
#![feature(try_trait_v2)]
#![feature(try_trait_v2_yeet)]
#![feature(try_trait_v2_residual)]
#![feature(decl_macro)]
#![feature(never_type)]
#![feature(iterator_try_collect)]
#![feature(negative_impls)]

use std::collections::HashSet;
use std::any::Any;
use std::fmt::Debug;
use std::hash::Hash;

use im::HashSet as IMHashSet;
use im::HashMap as IMHashMap;
use im::Vector as IMVector;

extern crate gccompat_derive;
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

mod gccow;
use gccow::*;

mod obj;
use obj::*;

#[doc(hidden)]
pub mod hidden {
    pub use crate::obj::*;
    pub use crate::gccow::{GcCow, GcCompat, mark_and_sweep};
}

pub use crate::nondet::*;
pub use crate::name::*;

/// The items from this module are automatically imported into minirust.
pub mod prelude {
    pub use crate::ret;
    pub use crate::Align;
    pub use crate::Size;
    pub use crate::Int;
    pub use crate::list::*;
    pub use crate::set::*;
    pub use crate::map::*;
    pub use crate::endianness::*;
    pub use crate::mutability::*;
    pub use crate::signedness::*;
    pub use crate::string::{String, format};
    pub use crate::nondet::{pick, predict};
    pub use crate::option::*;

    pub use std::hash::Hash;
    pub use std::fmt::Debug;
}
