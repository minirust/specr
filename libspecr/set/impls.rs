use crate::libspecr::*;

use std::hash::Hasher;
use std::fmt::{Formatter, Debug, Error};

impl<T: Obj> Default for Set<T> {
    fn default() -> Self {
        Self(GcCow::new(IMHashSet::new()))
    }
}

impl<T: Obj> Clone for Set<T> {
    fn clone(&self) -> Self { Self(self.0) }
}
impl<T: Obj> Copy for Set<T> {}

impl<T: Obj> Debug for Set<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

impl<T: Obj> GcCompat for Set<T> {
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl<T: Obj> GcCompat for IMHashSet<T> {
    fn points_to(&self, m: &mut HashSet<usize>) {
        for x in self.iter() {
            x.points_to(m);
        }
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl<T: Obj> Hash for Set<T> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.0.call_ref_unchecked(|s| s.hash(state))
    }
}

impl<T: Obj> PartialEq for Set<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

impl<T: Obj> Eq for Set<T> {}

