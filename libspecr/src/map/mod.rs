use std::collections::HashSet;

use im::HashMap as IMHashMap;

use crate::*;

mod func;
mod iter;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, GcCompat, PartialOrd, Ord)]
/// Garbage-collected hash map implementing `Copy`.
/// This implements `Ord` but the order is not meaningful; this is just so one can use `BTreeMap`s.
/// In particular, the order might differ across two runs of the same program.
pub struct Map<K: Obj, V: Obj>(pub(crate) GcCow<IMHashMap<K, V>>);

impl<K: Obj, V: Obj> GcCompat for IMHashMap<K, V> {
    fn points_to(&self, m: &mut HashSet<usize>) {
        for (k, v) in self.iter() {
            k.points_to(m);
            v.points_to(m);
        }
    }
}

// This is not #[derive]d, as this would wrongly require K: Default.
impl<K: Obj, V: Obj> Default for Map<K, V> {
    fn default() -> Self {
        Self(Default::default())
    }
}
