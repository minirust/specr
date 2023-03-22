use crate::*;

impl<T: Obj> Set<T> {
    /// Constructs an empty Set.
    pub fn new() -> Self {
        Self(Default::default())
    }

    /// Checks whether `t` is contained in `self`.
    pub fn contains(&self, t: T) -> bool {
        self.0.call_ref_unchecked(|s| {
            s.contains(&t)
        })
    }

    /// Inserts `t` into `self`.
    pub fn insert(&mut self, t: T) {
        self.0.mutate(|s| {
            s.insert(t);
        });
    }

    /// Removes `t` from `self`.
    pub fn remove(&mut self, t: T) {
        self.0.mutate(|s| {
            s.remove(&t);
        });
    }

    /// Returns the number of elements in `self`.
    pub fn len(self) -> Int {
        Int::from(self.0.call_ref_unchecked(|s| s.len()))
    }

    /// Returns `true` if the set contains no elements.
    pub fn is_empty(self) -> bool {
        self.0.call_ref_unchecked(|s| s.is_empty())
    }
}
