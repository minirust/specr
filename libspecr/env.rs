use crate::specr::*;

pub type Set<T> = HashSet<T>;
pub type Map<K, V> = HashMap<K, V>;

pub fn default<T: Default>() -> T {
	T::default()
}

pub fn pick<T>(_f: impl Fn(T) -> bool) -> Nondet<T> { todo!() }
pub fn predict<T>(_f: impl Fn(T) -> bool) -> Nondet<T> { todo!() }
