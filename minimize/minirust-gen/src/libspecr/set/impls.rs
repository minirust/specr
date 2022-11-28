use crate::libspecr::*;

use std::fmt::{Debug, Error, Formatter};
use std::hash::Hasher;

impl<T: 'static> Default for Set<T>
where
    T: GcCompat,
{
    fn default() -> Self {
        Self(GcCow::new(IMHashSet::new()))
    }
}

impl<T: 'static> Clone for Set<T> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}
impl<T: 'static> Copy for Set<T> {}

impl<T> Debug for Set<T>
where
    T: Hash + Debug + Eq + 'static,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

impl<T> GcCompat for Set<T>
where
    T: GcCompat + 'static,
{
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<T: GcCompat> GcCompat for IMHashSet<T>
where
    T: GcCompat + 'static,
{
    fn points_to(&self, m: &mut HashSet<usize>) {
        for x in self.iter() {
            x.points_to(m);
        }
    }
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<T> Hash for Set<T>
where
    T: GcCompat + Clone + Hash + Eq + 'static,
{
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        self.0.call_ref_unchecked(|s| s.hash(state))
    }
}
