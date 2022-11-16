use crate::libspecr::*;

use std::iter::FromIterator;
use std::ops::*;

impl<T: Clone + GcCompat> List<T> {
    pub fn new() -> List<T> {
        List(gccow_new(IMVector::new()))
    }

    pub fn len(&self) -> BigInt {
        self.0.call_ref(|v| BigInt::from(v.len()))
    }

    pub fn first(&self) -> Option<T> {
        self.0.call_ref(|v| v.front().cloned())
    }

    pub fn last(&self) -> Option<T> {
        self.0.call_ref(|v| v.last().cloned())
    }

    pub fn get(&self, i: BigInt) -> Option<T> {
        let i = bigint_to_usize(i);
        self.0.call_ref(|v| v.get(i).cloned())
    }

    pub fn get_unchecked(&self, i: BigInt) -> T {
        self.get(i).unwrap()
    }

    pub fn push(&mut self, t: T) {
        self.0.call_mut(|v| v.push_back(t));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.call_mut(|v| v.pop_back())
    }

    pub fn chunks(&self, chunk_size: BigInt) -> impl Iterator<Item=List<T>> where Self: Copy {
        let s = *self;
        let mut i = BigInt::zero();
        std::iter::from_fn(move || {
            let size = chunk_size.min(s.len() - i);
            if size <= 0 { return None; }
            let val = s.subslice_with_length(i, size);
            i += chunk_size;
            Some(val)
        })
    }

    pub fn reverse(&mut self) {
        *self = self.0.call_ref(|v| v.iter().cloned().rev().collect());
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

