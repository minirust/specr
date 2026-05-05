use std::collections::HashSet;

use im::HashSet as IMHashSet;

use crate::*;

mod func;
mod iter;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, GcCompat, PartialOrd, Ord)]
/// Garbage-collected hash set implementing `Copy`.
/// This implements `Ord` but the order is not meaningful; this is just so one can use `BTreeMap`s.
/// In particular, the order might differ across two runs of the same program.
pub struct Set<T: Obj>(pub(crate) GcCow<IMHashSet<T>>);

impl<T: Obj> GcCompat for IMHashSet<T> {
    fn points_to(&self, m: &mut HashSet<usize>) {
        for x in self.iter() {
            x.points_to(m);
        }
    }
}

// This is not #[derive]d, as this would wrongly require T: Default.
impl<T: Obj> Default for Set<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
