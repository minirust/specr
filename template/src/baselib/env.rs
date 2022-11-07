use crate::baselib::*;

pub type List<T> = Vec<T>;
pub type Set<T> = HashSet<T>;
pub type Map<K, V> = HashMap<K, V>;

pub fn default<T: Default>() -> T {
	T::default()
}

#[macro_export]
macro_rules! list {
	() => { vec![] };
	($start:expr $(,$a:expr)*) => { vec![$start $(,$a)* ] };
	($a:expr ; $b:expr) => { vec![$a ; $b] };
}

pub fn pick<T>(f: impl Fn(T) -> bool) -> Nondet<T> { todo!() }
pub fn predict<T>(f: impl Fn(T) -> bool) -> Nondet<T> { todo!() }
