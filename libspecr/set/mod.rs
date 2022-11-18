use crate::libspecr::*;

mod func;

use std::fmt::{Formatter, Debug, Error};

pub struct Set<T>(GcCow<IMHashSet<T>>);

impl<T> Clone for Set<T> {
    fn clone(&self) -> Self { Set(self.0) }
}
impl<T> Copy for Set<T> {}

impl<T> Debug for Set<T> where T: Hash + Debug + Eq {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

impl<T> GcCompat for Set<T> where T: GcCompat{
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl<T: GcCompat> GcCompat for IMHashSet<T> where T: GcCompat {
    fn points_to(&self, m: &mut HashSet<usize>) {
        for x in self.iter() {
            x.points_to(m);
        }
    }
    fn as_any(&self) -> &dyn Any { self }
}
