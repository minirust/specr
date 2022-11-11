use std::iter::FromIterator;
use std::slice::Chunks;
use std::ops::*;

use crate::specr::BigInt;

#[derive(Clone)]
pub struct List<T>(pub(in crate::specr) Vec<T>);

pub macro list {
	() => { List::new() },
	($start:expr $(,$a:expr)*) => { crate::specr::hidden::vec_to_list(vec![$start $(,$a)* ]) },
	($a:expr ; $b:expr) => {
        crate::specr::hidden::vec_to_list(
            vec![$a;
                crate::specr::hidden::bigint_to_usize(BigInt::from($b))
            ]
        )
    },
}

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

impl<T> Index<BigInt> for List<T> {
    type Output = T;

    fn index(&self, other: BigInt) -> &T {
        let other = crate::specr::hidden::bigint_to_usize(other);
        &self.0[other]
    }
}

impl<T> IndexMut<BigInt> for List<T> {
    fn index_mut(&mut self, other: BigInt) -> &mut T {
        let other = crate::specr::hidden::bigint_to_usize(other);
        &mut self.0[other]
    }
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List(Vec::new())
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.0.iter()
    }

    pub fn len(&self) -> BigInt {
        BigInt::from(self.0.len())
    }

    pub fn last(&self) -> Option<&T> {
        self.0.last()
    }

    pub fn push(&mut self, t: T) {
        self.0.push(t);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.0.last_mut()
    }

    pub fn chunks(&self, chunk_size: BigInt) -> Chunks<'_, T> {
        let i = crate::specr::hidden::bigint_to_usize(chunk_size);
        self.0.chunks(i)
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
