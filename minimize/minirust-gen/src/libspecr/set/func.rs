use crate::libspecr::*;

impl<T> Set<T>
where
    T: GcCompat + Clone + Hash + Eq + 'static,
{
    pub fn contains(&self, t: T) -> bool {
        self.0.call_ref_unchecked(|s| s.contains(&t))
    }

    pub fn insert(&mut self, t: T) {
        self.0.mutate(|s| {
            s.insert(t);
        });
    }

    pub fn remove(&mut self, t: T) {
        self.0.mutate(|s| {
            s.remove(&t);
        });
    }
}
