use crate::specr::*;
use crate::specr::gccow::{GcCow, GcCompat};
use im::hashset::HashSet;
use im::hashmap::HashMap;

use std::any::Any;

pub fn default<T: Default>() -> T {
	T::default()
}

pub fn pick<T>(_f: impl Fn(T) -> bool) -> Nondet<T> { todo!() }
pub fn predict<T>(_f: impl Fn(T) -> bool) -> Nondet<T> { todo!() }

#[derive(Copy, Clone)]
pub enum Endianness {
    LittleEndian,
    BigEndian
}
pub use Endianness::*;
