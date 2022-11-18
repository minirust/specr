use crate::libspecr::*;

impl<K: GcCompat + Clone + Hash + Eq, V: GcCompat + Clone> Map<K, V> {
    pub fn get(&self, k: K) -> Option<V> {
        self.0.call_ref_unchecked(|m| m.get(&k).cloned())
    }

    pub fn index_at(&self, k: K) -> V {
        self.get(k).unwrap()
    }
}
