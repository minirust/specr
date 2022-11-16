use crate::specr::*;
use crate::specr::gccow::GcCow;
use im::hashset::HashSet;
use im::hashmap::HashMap;

#[derive(Clone)]
pub struct Set<T>(GcCow<HashSet<T>>);

impl<T> Copy for Set<T> {}

#[derive(Clone)]
pub struct Map<K, V>(GcCow<HashMap<K, V>>);

impl<K, V> Copy for Map<K, V> {}

#[derive(Copy, Clone, Debug, Hash)]
pub struct String(pub(in crate::specr) GcCow<std::string::String>);

pub fn default<T: Default>() -> T {
	T::default()
}

pub fn pick<T>(_f: impl Fn(T) -> bool) -> Nondet<T> { todo!() }
pub fn predict<T>(_f: impl Fn(T) -> bool) -> Nondet<T> { todo!() }
