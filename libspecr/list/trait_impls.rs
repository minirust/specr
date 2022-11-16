// contains trait implementations for List.
// An exception to this are Index(Mut) impls, those are in the index module.

use std::iter::FromIterator;
use std::slice::Chunks;
use std::ops::{Deref, DerefMut};

use im::vector::Vector;

use crate::specr::list::List;
use crate::specr::gccow::{GcCompat, gccow_new};

impl<A: GcCompat + Clone> FromIterator<A> for List<A> {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = A> {
        let v: Vector<A> = iter.into_iter().collect();
        List(gccow_new(v))
    }
}
