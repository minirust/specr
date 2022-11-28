use crate::libspecr::*;

use std::iter::FromIterator;
use std::ops::*;

use im::vector::ConsumingIter;

impl<T> List<T>
where
    T: GcCompat + Clone + 'static,
{
    pub fn iter(&self) -> ConsumingIter<T> {
        self.into_iter()
    }
}

impl<T> IntoIterator for List<T>
where
    T: GcCompat + Clone + 'static,
{
    type Item = T;
    type IntoIter = ConsumingIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.call_ref_unchecked(|v| v.clone().into_iter())
    }
}

impl<A: GcCompat + Clone + 'static> FromIterator<A> for List<A> {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = A>,
    {
        let v: IMVector<A> = iter.into_iter().collect();
        List(GcCow::new(v))
    }
}
