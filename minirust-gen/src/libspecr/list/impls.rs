use crate::libspecr::*;

use std::fmt::{Debug, Error, Formatter};
use std::hash::Hasher;

impl<T: 'static> Default for List<T>
where
    T: GcCompat + Clone,
{
    fn default() -> Self {
        Self(GcCow::new(IMVector::new()))
    }
}

impl<T: 'static> Clone for List<T> {
    fn clone(&self) -> Self {
        List(self.0)
    }
}
impl<T: 'static> Copy for List<T> {}

impl<T> Debug for List<T>
where
    T: Debug + Clone + 'static,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

impl<T: Clone + 'static> GcCompat for IMVector<T>
where
    T: GcCompat,
{
    fn points_to(&self, m: &mut HashSet<usize>) {
        for i in self.iter() {
            i.points_to(m);
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<T: 'static> GcCompat for List<T>
where
    T: GcCompat + Clone,
{
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<T: 'static> PartialEq for List<T>
where
    T: GcCompat + Clone + PartialEq,
{
    fn eq(&self, other: &List<T>) -> bool {
        self.0.get() == other.0.get()
    }
}

impl<T> Eq for List<T> where T: GcCompat + Clone + Eq + 'static {}

impl<T> Hash for List<T>
where
    T: GcCompat + Clone + Hash + 'static,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.0.call_ref_unchecked(|v| v.hash(state))
    }
}
