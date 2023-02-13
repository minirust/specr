use crate::*;

use im::hashset::ConsumingIter;

impl<T: Obj> Set<T> {
    /// An iterator over all elements of self.
    pub fn iter(self) -> ConsumingIter<T> {
        self.into_iter()
    }
}

impl<T: Obj> IntoIterator for Set<T> {
    type Item = T;
    type IntoIter = ConsumingIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.call_ref_unchecked(|s| s.clone().into_iter())

    }
}
