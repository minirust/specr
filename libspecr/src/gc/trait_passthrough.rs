use std::fmt::{Debug, Formatter, Error};
use std::hash::{Hash, Hasher};

use crate::*;

impl<T> Debug for GcCow<T> where T: Debug + GcCompat + Clone {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.call_ref_unchecked(|x| x.fmt(f))
    }
}

impl<T> Hash for GcCow<T> where T: Hash + GcCompat + Clone {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.call_ref_unchecked(|x| x.hash(state))
    }
}

impl<T> PartialEq for GcCow<T> where T: GcCompat + PartialEq + Clone {
    fn eq(&self, other: &Self) -> bool {
        self.call_ref1_unchecked(*other, |x, y| x == y)
    }
}

impl<T> Eq for GcCow<T> where T: GcCompat + Eq + Clone {}

impl<T> Default for GcCow<T> where T: Default + GcCompat {
    fn default() -> Self {
        Self::new(T::default())
    }
}
