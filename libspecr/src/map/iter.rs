use im::HashMap as IMHashMap;
use im::hashmap::ConsumingIter;

use crate::*;

impl<K: Obj, V: Obj> Map<K, V> {
    /// Returns an iterator over all (key, value) pairs.
    pub fn iter(self) -> ConsumingIter<(K, V)> {
        self.into_iter()
    }
}

impl<K: Obj, V: Obj> IntoIterator for Map<K, V> {
    type Item = (K, V);
    type IntoIter = ConsumingIter<(K, V)>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.call_ref_unchecked(|v| v.clone().into_iter())
    }
}

impl<K: Obj, V: Obj> FromIterator<(K, V)> for Map<K, V> {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = (K, V)> {
        let v: IMHashMap<K, V> = iter.into_iter().collect();
        Map(GcCow::new(v))
    }
}
