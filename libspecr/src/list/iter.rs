use crate::*;

use std::iter::FromIterator;

use im::vector::ConsumingIter;

impl<T: Obj> List<T> {
    pub fn iter(&self) -> ConsumingIter<T> {
        self.into_iter()
    }
}

impl<T: Obj> IntoIterator for List<T> {
    type Item = T;
    type IntoIter = ConsumingIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.call_ref_unchecked(|v| v.clone().into_iter())
    }
}

impl<A: Obj> FromIterator<A> for List<A> {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = A> {
        let v: IMVector<A> = iter.into_iter().collect();
        List(GcCow::new(v))
    }
}
