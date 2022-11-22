use crate::libspecr::*;

use im::hashmap::ConsumingIter;

impl<K, V> Map<K, V>
where
    K: GcCompat + Clone + Hash + Eq + 'static,
    V: GcCompat + Clone + 'static,
{
    pub fn iter(self) -> ConsumingIter<(K, V)> {
        self.into_iter()
    }
}

impl<K, V> IntoIterator for Map<K, V>
where
    K: GcCompat + Clone + Hash + Eq + 'static,
    V: GcCompat + Clone + 'static,
{
    type Item = (K, V);
    type IntoIter = ConsumingIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.call_ref_unchecked(|v| v.clone().into_iter())
    }
}
