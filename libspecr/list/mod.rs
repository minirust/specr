use crate::libspecr::*;

use std::fmt::{Formatter, Debug, Error};

mod iter;
mod func;

pub struct List<T>(pub(in crate::libspecr) GcCow<IMVector<T>>);

pub macro list {
	() => { List::new() },
	($start:expr $(,$a:expr)*) => { [$start $(,$a)* ].into_iter().collect::<List<_>>() },
	($a:expr ; $b:expr) => { list_from_elem($a, BigInt::from($b)) },
}

impl<T> Clone for List<T> {
    fn clone(&self) -> Self { List(self.0) }
}
impl<T> Copy for List<T> {}

impl<T> Debug for List<T> where T: Debug + Clone {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

impl<T: Clone> GcCompat for IMVector<T> where T: GcCompat {
    fn points_to(&self, m: &mut HashSet<usize>) {
        for i in self.iter() {
            i.points_to(m);
        }
    }
    fn as_any(&self) -> &dyn Any { self}
}

impl<T> GcCompat for List<T> where T: GcCompat + Clone {
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self}
}
