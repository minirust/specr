use std::iter::FromIterator;
use std::slice::Chunks;
use std::ops::*;

use im::vector::Vector;

use crate::specr::BigInt;
use crate::specr::hidden::bigint_to_usize;
use crate::specr::list::List;
use crate::specr::gccow::{GcCompat, gccow_new};

impl<T: Clone + GcCompat> List<T> {
    pub fn new() -> List<T> {
        List(gccow_new(Vector::new()))
    }

    pub fn iter(&self) -> List<T> {
        // TODO reference problem
        // self.0.call_ref(|v| v.iter())
        *self
    }

    pub fn len(&self) -> BigInt {
        self.0.call_ref(|v| BigInt::from(v.len()))
    }

    pub fn last(&self) -> Option<&T> {
        self.0.call_ref(|v| v.last())
    }

    pub fn push(&mut self, t: T) {
        self.0.call_mut(|v| v.push_back(t));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.call_mut(|v| v.pop_back())
    }

    pub fn chunks(&self, chunk_size: BigInt) -> Chunks<'_, T> {
        // TODO Vector::chunks does not exist.
        let i = bigint_to_usize(chunk_size);
        self.0.call_ref(|v| v.chunks(i))
    }

    pub fn subslice_with_length(&self, start: BigInt, length: BigInt) -> List<T> {
        let start = bigint_to_usize(start);
        let length = bigint_to_usize(length);
        let end = start+length;

        self.0.call_ref(|v|
            List(gccow_new(v.clone().slice(start..end)))
        )
    }
}

