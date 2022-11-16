use crate::specr::*;
use crate::specr::gccow::GcCow;

pub type Set<T> = HashSet<T>;
pub type Map<K, V> = HashMap<K, V>;

#[derive(Copy, Clone, Debug, Hash)]
pub struct String(pub(in crate::specr) GcCow<std::string::String>);

pub fn default<T: Default>() -> T {
	T::default()
}

pub fn pick<T>(_f: impl Fn(T) -> bool) -> Nondet<T> { todo!() }
pub fn predict<T>(_f: impl Fn(T) -> bool) -> Nondet<T> { todo!() }
