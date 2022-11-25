use crate::libspecr::*;

mod iter;
mod func;
mod impls;

pub struct List<T: Obj>(pub(in crate::libspecr) GcCow<IMVector<T>>);

pub macro list {
	() => { List::new() },
	($start:expr $(,$a:expr)*) => { [$start $(,$a)* ].into_iter().collect::<List<_>>() },
	($a:expr ; $b:expr) => { list_from_elem($a, BigInt::from($b)) },
}
