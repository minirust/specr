use std::iter::FromIterator;
use std::slice::Chunks;
use std::ops::*;

use crate::specr::BigInt;
use crate::specr::hidden::{bigint_to_usize, vec_to_list};
use crate::specr::list::List;

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
        let i = bigint_to_usize(chunk_size);
        self.0.chunks(i)
    }

    pub fn subslice_with_length(&self, start: BigInt, length: BigInt) -> &[T] {
        let start = bigint_to_usize(start);
        let length = bigint_to_usize(length);

        &self.0[start..][..length]
    }
}

