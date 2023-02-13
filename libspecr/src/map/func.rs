use crate::*;

impl<K: Obj, V: Obj> Map<K, V> {
    /// Creates an empty Map.
    pub fn new() -> Self {
        Self(Default::default())
    }

    /// Returns the value to the key `k`.
    /// Returns `None`, if the key `k` is not in the Map.
    pub fn get(&self, k: K) -> Option<V> {
        self.0.call_ref_unchecked(|m| m.get(&k).cloned())
    }

    /// The indexing operator:
    /// specr translates `a[b]` to `a.index_at(b)`.
    pub fn index_at(&self, k: K) -> V {
        self.get(k).unwrap()
    }

    /// Removes `k` from the map.
    /// If the pair (k, v)` was in the map, `Some(v)` is returned.
    /// Otherwise `None` is returned.
    pub fn remove(&mut self, k: K) -> Option<V> {
        self.0.mutate(|m| {
            m.remove(&k)
        })
    }

    /// Checks whether `self` contains `k`.
    pub fn contains_key(&self, k: K) -> bool {
        self.0.call_ref_unchecked(|m| {
            m.contains_key(&k)
        })
    }

    /// Insert a key/value mapping into a map.
    /// If the map already has a mapping for the given key, the previous value is overwritten.
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.0.mutate(|m| {
            m.insert(k, v)
        })
    }

    /// Like `insert`, but fails if `k` was already in the map.
    pub fn try_insert(&mut self, k: K, v: V) -> Result<(), ()> {
        if self.contains_key(k.clone()) {
            return Err(());
        }

        self.insert(k, v);

        Ok(())
    }

    /// An iterator over all keys.
    pub fn keys(self) -> impl Iterator<Item=K> {
        self.into_iter().map(|(k, _)| k)
    }

    /// An iterator over all values.
    pub fn values(self) -> impl Iterator<Item=V> {
        self.into_iter().map(|(_, v)| v)
    }
}
