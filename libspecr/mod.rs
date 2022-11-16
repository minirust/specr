use std::collections::{HashSet, HashMap};
use std::any::Any;

use im::HashSet as IMHashSet;
use im::HashMap as IMHashMap;
use im::Vector as IMVector;

mod bigint;
pub use bigint::*;

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

mod endianness;
pub use endianness::*;

// code that belongs to minirust, but is to verbose there.
// no other modules are allowed to use things defined in MiniRust.
mod inject;

mod ret;
pub use ret::*;

mod gccow;
pub use gccow::*;

pub mod hidden;
pub use hidden::*;

// publicly accessible items from libspecr.
pub mod public {
    pub use crate::libspecr::name::*;
    pub use crate::libspecr::bigint::*;
    pub use crate::libspecr::nondet::*;

    pub use crate::libspecr::hidden;

    // auto-included items from libspecr.
    pub mod prelude {
        pub use crate::libspecr::BigInt;
        pub use std::hash::Hash;
        pub use std::fmt::Debug;
        pub use crate::libspecr::GcCompat;
        pub use crate::libspecr::list::*;
        pub use crate::libspecr::set::*;
        pub use crate::libspecr::map::*;
        pub use crate::libspecr::endianness::*;
        pub use crate::libspecr::string::*;

        pub fn default<T: Default>() -> T { T::default() }
        pub fn pick<T>(_f: impl Fn(T) -> bool) -> crate::libspecr::Nondet<T> { todo!() }
        pub fn predict<T>(_f: impl Fn(T) -> bool) -> crate::libspecr::Nondet<T> { todo!() }
    }
}
