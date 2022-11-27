#![feature(int_roundings)]
#![feature(const_trait_impl)]
#![feature(try_trait_v2)]
#![feature(try_trait_v2_yeet)]
#![feature(try_trait_v2_residual)]
#![feature(yeet_expr)]
#![feature(iterator_try_collect)]
#![feature(never_type)]
#![feature(decl_macro)]
#![feature(map_try_insert)]

use std::collections::{HashSet, HashMap};
use std::any::Any;
use std::fmt::Debug;
use std::hash::Hash;

use im::HashSet as IMHashSet;
use im::HashMap as IMHashMap;
use im::Vector as IMVector;

pub mod int;
pub use int::*;

pub mod ndresult;
pub use ndresult::*;

pub mod size;
pub use size::*;

pub mod align;
pub use align::*;

pub mod list;
pub use list::*;

pub mod set;
pub use set::*;

pub mod map;
pub use map::*;

#[macro_use]
pub mod string;
pub use string::*;

pub mod name;
pub use name::*;

pub mod nondet;
pub use nondet::*;

pub mod endianness;
pub use endianness::*;

pub mod ret;
pub use ret::*;

pub mod signedness;
pub use signedness::*;

pub mod gccow;
pub use gccow::*;

pub mod obj;
pub use obj::*;

pub mod hidden;
pub use hidden::*;

pub mod public {
    pub use crate::hidden;
    pub use crate::GcCompat;
    pub use crate::nondet::*;
    pub use crate::int::*;
    pub use crate::name::*;
    pub use crate::signedness::*;

    // auto-included items
    pub mod prelude {
        pub use crate::Align;
        pub use crate::Size;
        pub use crate::GcCompat;
        pub use crate::Int;
        pub use crate::list::*;
        pub use crate::set::*;
        pub use crate::map::*;
        pub use crate::endianness::*;
        pub use crate::string::{String, format};

        pub use std::hash::Hash;
        pub use std::fmt::Debug;

        pub fn default<T: Default>() -> T { T::default() }
        pub fn pick<T>(_f: impl Fn(T) -> bool) -> crate::MyNdResult<T> { todo!() }
        pub fn predict<T>(_f: impl Fn(T) -> bool) -> crate::MyNdResult<T> { todo!() }

    }
}
