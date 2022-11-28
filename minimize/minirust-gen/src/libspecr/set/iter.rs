use crate::libspecr::*;

use im::hashset::ConsumingIter;

impl<T> Set<T>
where
    T: GcCompat + Clone + Hash + Eq + 'static,
{
    pub fn iter(self) -> ConsumingIter<T> {
        self.into_iter()
    }
}

impl<T> IntoIterator for Set<T>
where
    T: GcCompat + Clone + Hash + Eq + 'static,
{
    type Item = T;
    type IntoIter = ConsumingIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.call_ref_unchecked(|s| s.clone().into_iter())
    }
}
