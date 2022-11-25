use crate::libspecr::*;

use std::hash::Hasher;
use std::fmt::{Formatter, Debug, Error};

impl<T: Obj> Default for List<T> {
    fn default() -> Self {
        Self(GcCow::new(IMVector::new()))
    }
}

impl<T: Obj> Clone for List<T> {
    fn clone(&self) -> Self { List(self.0) }
}
impl<T: Obj> Copy for List<T> {}

impl<T: Obj> Debug for List<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.fmt(f)
    }
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

impl<T: Obj> PartialEq for List<T> {
    fn eq(&self, other: &List<T>) -> bool {
        self.0.get() == other.0.get()
    }
}

impl<T: Obj> Eq for List<T> {}

impl<T: Obj> Hash for List<T> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.0.call_ref_unchecked(|v| v.hash(state))
    }
}
