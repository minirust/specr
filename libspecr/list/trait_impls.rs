// contains trait implementations for List.
// An exception to this are Index(Mut) impls, those are in the index module.

use std::iter::FromIterator;
use std::slice::Chunks;
use std::ops::{Deref, DerefMut};

use crate::specr::list::List;

impl<T> IntoIterator for List<T> {
    type IntoIter = <Vec::<T> as IntoIterator>::IntoIter;
    type Item = T;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<A> FromIterator<A> for List<A> {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = A> {
        let v: Vec<A> = iter.into_iter().collect();
        List(v)
    }
}

impl<T> Deref for List<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        &*self.0
    }
}

impl<T> DerefMut for List<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        &mut *self.0
    }
}
