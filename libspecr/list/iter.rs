use crate::libspecr::*;

use std::iter::FromIterator;
use std::ops::*;

struct ListIter<T> {
    list: List<T>,
    idx: BigInt,
}

impl<T: GcCompat + Clone> Iterator for ListIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let out = self.list.get(self.idx);
        if out.is_some() { self.idx += 1; }
        out
    }
}

impl<T> List<T> {
    pub fn iter(&self) -> ListIter<T> where Self: Copy {
        ListIter { list: *self, idx: BigInt::zero() }
    }
}

impl<T: GcCompat + Clone> IntoIterator for List<T> where Self: Copy {
    type Item = T;
    type IntoIter = ListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<A: GcCompat + Clone> FromIterator<A> for List<A> {
    fn from_iter<T>(iter: T) -> Self where T: IntoIterator<Item = A> {
        let v: IMVector<A> = iter.into_iter().collect();
        List(gccow_new(v))
    }
}
