use crate::*;

use std::fmt::{Formatter, Debug, Error};
use std::hash::{Hash, Hasher};

impl<T> Debug for GcCow<T> where T: Debug + GcCompat + Clone {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.get().fmt(f)
    }
}

impl<T> Hash for GcCow<T> where T: Hash + GcCompat + Clone {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.get().hash(state)
    }
}

impl<T> PartialEq for GcCow<T> where T: GcCompat + PartialEq + Clone {
    fn eq(&self, other: &Self) -> bool {
        self.get() == other.get()
    }
}

impl<T> Eq for GcCow<T> where T: GcCompat + Eq + Clone {}


impl<T> Default for GcCow<T> where T: Default + GcCompat {
    fn default() -> Self {
        Self::new(T::default())
    }
}

