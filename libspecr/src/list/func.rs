use crate::*;

use std::iter::FromIterator;
use std::ops::*;

impl<T: Obj> List<T> {
    pub fn new() -> List<T> {
        List(GcCow::new(IMVector::new()))
    }

    pub fn len(&self) -> Int {
        Int::from(self.0.call_ref_unchecked(|v| v.len()))
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn first(&self) -> Option<T> {
        self.0.call_ref_unchecked(|v| v.front().cloned())
    }

    pub fn last(&self) -> Option<T> {
        self.0.call_ref_unchecked(|v| v.last().cloned())
    }

    pub fn mutate_at<O: Obj>(&mut self, i: Int, f: impl FnOnce(&mut T) -> O) -> O {
        let i = int_to_usize(i);
        self.0.mutate(|v| f(&mut v[i]))
    }

    pub fn get(&self, i: Int) -> Option<T> {
        let i = int_to_usize(i);
        self.0.call_ref_unchecked(|v| v.get(i).cloned())
    }

    pub fn index_at(&self, i: impl Into<Int>) -> T {
        self.get(i.into()).unwrap()
    }

    pub fn push(&mut self, t: T) {
        self.0.mutate(|v| v.push_back(t));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.mutate(|v| v.pop_back())
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.0.mutate(|v| v.pop_front())
    }

    pub fn chunks(&self, chunk_size: Int) -> impl Iterator<Item=List<T>> {
        let s = *self;
        let mut i = Int::from(0);
        std::iter::from_fn(move || {
            let size = chunk_size.min(s.len() - i);
            if size <= 0 { return None; }
            let val = s.subslice_with_length(i, size);
            i += chunk_size;
            Some(val)
        })
    }

    pub fn reverse(&mut self) {
        let v: IMVector<T> = self.0.call_ref_unchecked(|v| v.iter().cloned().rev().collect());
        *self = List(GcCow::new(v));
    }

    pub fn subslice_with_length(&self, start: Int, length: Int) -> List<T> {
        let start = int_to_usize(start);
        let length = int_to_usize(length);

        // exclusive end
        let end = start + length;

        let v: IMVector<T> = self.0.call_ref_unchecked(|v| v.clone().slice(start..end));

        List(GcCow::new(v))
    }

    pub fn write_subslice_at_index(&mut self, start: Int, src: List<T>) {
        // exclusive end
        let end = start + src.len();

        if end > self.len() {
            panic!("`write_subslice_at_index`: trying to write out of range!");
        }

        let start = int_to_usize(start);
        let end = int_to_usize(end);

        self.0.call_mut1_unchecked(src.0, |s, o| {
            let a = s.clone().slice(0..start);
            let b = o.clone();
            let c = s.clone().slice(end..);

            *s = a + b + c;
        });
    }

    // note that `f` could modify the GC_STATE.
    pub fn sort_by_key<K: Obj + Ord>(&mut self, mut f: impl FnMut(T) -> K) {
        // I think we don't lose too much performance by going to a Vec here, as
        // (1.) it's only used once at all, for tuple types to order their fields.
        // (2.) At least in the case of doing actual notable amounts of sorting `im` needs to fully re-create the vec anyways.
        let mut vec: Vec<T> = self.iter().collect();
        vec.sort_by_key(|t| f(*t));
        *self = vec.into_iter().collect();
    }
}

