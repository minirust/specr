use crate::libspecr::*;

mod iter;
mod func;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Default, Debug)]
pub struct List<T: Obj>(pub(in crate::libspecr) GcCow<IMVector<T>>);

pub macro list {
	() => { List::new() },
	($start:expr $(,$a:expr)*) => { [$start $(,$a)* ].into_iter().collect::<List<_>>() },
	($a:expr ; $b:expr) => { list_from_elem($a, BigInt::from($b)) },
}

impl<T: Obj> GcCompat for IMVector<T> {
    fn points_to(&self, m: &mut HashSet<usize>) {
        for i in self.iter() {
            i.points_to(m);
        }
    }
    fn as_any(&self) -> &dyn Any { self}
}

impl<T: Obj> GcCompat for List<T> {
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self}
}

