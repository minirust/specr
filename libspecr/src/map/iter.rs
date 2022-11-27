use crate::*;

use im::hashmap::ConsumingIter;

impl<K: Obj, V: Obj> Map<K, V> {
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
