use crate::libspecr::*;

use std::hash::Hasher;
use std::fmt::{Formatter, Debug, Error};

impl<K: Obj, V: Obj> Default for Map<K, V> {
    fn default() -> Self {
        Self(GcCow::new(IMHashMap::new()))
    }
}

impl<K: Obj, V: Obj> Clone for Map<K, V> {
    fn clone(&self) -> Self { Self(self.0) }
}
impl<K: Obj, V: Obj> Copy for Map<K, V> {}

impl<K: Obj, V: Obj> Debug for Map<K, V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.0.fmt(f)
    }
}

impl<K: Obj, V: Obj> GcCompat for Map<K, V> {
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl<K: Obj, V: Obj> GcCompat for IMHashMap<K, V> {
    fn points_to(&self, m: &mut HashSet<usize>) {
        for (k, v) in self.iter() {
            k.points_to(m);
            v.points_to(m);
        }
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl<K: Obj, V: Obj> PartialEq for Map<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0.get() == other.0.get()
    }
}

impl<K: Obj, V: Obj> Eq for Map<K, V> {}

impl<K: Obj, V: Obj> Hash for Map<K, V> {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.0.call_ref_unchecked(|m| m.hash(state))
    }
}
