use crate::libspecr::*;

mod func;
mod impls;
mod iter;

pub struct List<T>(pub(in crate::libspecr) GcCow<IMVector<T>>);

pub macro list {
	() => { List::new() },
	($start:expr $(,$a:expr)*) => { [$start $(,$a)* ].into_iter().collect::<List<_>>() },
	($a:expr ; $b:expr) => { list_from_elem($a, BigInt::from($b)) },
}
