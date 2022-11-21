use crate::libspecr::*;

use std::fmt::{Formatter, Debug, Error};

impl<T> Default for List<T> where T: GcCompat + Clone {
    fn default() -> Self {
        Self(GcCow::new(IMVector::new()))
    }
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

impl<T> PartialEq for List<T> where T: GcCompat + Clone + PartialEq {
    fn eq(&self, other: &List<T>) -> bool {
        self.0.get() == other.0.get()
    }
}

impl<T> Eq for List<T> where T: GcCompat + Clone + Eq {}
