use std::collections::{HashSet, HashMap};

#[macro_use]
mod mirror;
pub use mirror::*;

mod bigint;
pub use bigint::BigInt;

pub mod prelude {
    pub use crate::baselib::mirror::*;
    pub use super::{
        BigInt,
        List,
        Set,
        Map,
        pick,
        predict,
        default,
    };
}

pub type List<T> = Vec<T>;
pub type Set<T> = HashSet<T>;
pub type Map<K, V> = HashMap<K, V>;

pub fn default<T: Default>() -> T {
	T::default()
}

pub struct ArgAbi;

pub type BbName = String;
pub type LocalName = String;
pub type FnName = String;

pub fn pick<T>(f: impl Fn(T) -> bool) -> Nondet<T> { todo!() }
pub fn predict<T>(f: impl Fn(T) -> bool) -> Nondet<T> { todo!() }

#[macro_export]
macro_rules! list {
	() => { vec![] };
	($start:expr $(,$a:expr)*) => { vec![$start $(,$a)* ] };
	($a:expr ; $b:expr) => { vec![$a ; $b] };
}
