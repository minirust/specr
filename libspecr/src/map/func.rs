use crate::*;

impl<K: Obj, V: Obj> Map<K, V> {
    pub fn get(&self, k: K) -> Option<V> {
        self.0.call_ref_unchecked(|m| m.get(&k).cloned())
    }

    pub fn index_at(&self, k: K) -> V {
        self.get(k).unwrap()
    }

    pub fn remove(&mut self, k: K) -> Option<V> {
        self.0.mutate(|m| {
            m.remove(&k)
        })
    }

    pub fn contains_key(&self, k: K) -> bool {
        self.0.call_ref_unchecked(|m| {
            m.contains_key(&k)
        })
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.0.mutate(|m| {
            m.insert(k, v)
        })
    }

    pub fn try_insert(&mut self, k: K, v: V) -> Result<(), ()> {
        if self.contains_key(k.clone()) {
            return Err(());
        }

        self.insert(k, v);

        Ok(())
    }

    pub fn keys(self) -> impl Iterator<Item=K> {
        self.into_iter().map(|(k, _)| k)
    }

    pub fn values(self) -> impl Iterator<Item=V> {
        self.into_iter().map(|(_, v)| v)
    }
}
