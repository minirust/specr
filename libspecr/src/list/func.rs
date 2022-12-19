use crate::*;

use std::ops::*;

impl<T: Obj> List<T> {
    pub fn new() -> Self {
        Self(Default::default())
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
        let i = i.try_to_usize().expect("List::mutate_at: index out of range of `usize`!");
        self.0.mutate(|v| f(&mut v[i]))
    }

    pub fn get(&self, i: Int) -> Option<T> {
        let i = i.try_to_usize().expect("List::get: index out of range of `usize`!");
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

    pub fn chunks(&self, chunk_size: Int) -> List<List<T>> {
        let s = *self;
        let mut i = Int::from(0);
        std::iter::from_fn(move || {
            let size = chunk_size.min(s.len() - i);
            if size <= 0 { return None; }
            let val = s.subslice_with_length(i, size);
            i += chunk_size;
            Some(val)
        }).collect()
    }

    pub fn reverse(&mut self) {
        let v: IMVector<T> = self.0.call_ref_unchecked(|v| v.iter().cloned().rev().collect());
        *self = List(GcCow::new(v));
    }

    pub fn subslice_with_length(&self, start: Int, length: Int) -> List<T> {
        let start = start.try_to_usize().expect("List::subslice_with_length: `start` out of range of `usize`!");
        let length = length.try_to_usize().expect("List::subslice_with_length: `length` out of range of `usize`!");

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

        let start = start.try_to_usize().expect("List::subslice_with_length: `start` out of range of `usize`!");
        let end = end.try_to_usize().expect("List::subslice_with_length: `end` out of range of `usize`!");

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

    pub fn zip<T2: Obj>(self, other: List<T2>) -> List<(T, T2)> {
        self.iter().zip(other.iter()).collect()
    }

    pub fn any(self, f: impl FnMut(T) -> bool) -> bool {
        self.iter().any(f)
    }

    pub fn all(self, f: impl FnMut(T) -> bool) -> bool {
        self.iter().all(f)
    }

    pub fn map<O: Obj>(self, f: impl FnMut(T) -> O) -> List<O> {
        self.iter().map(f).collect()
    }

    pub fn flat_map<O: Obj>(self, f: impl FnMut(T) -> List<O>) -> List<O> {
        self.iter().flat_map(f).collect()
    }

    pub fn try_map<O: Obj, E>(self, f: impl FnMut(T) -> E) -> <<E as Try>::Residual as Residual<List<O>>>::TryType
        where E: Try<Output=O>,
              <E as Try>::Residual: Residual<List<O>>,
    {
        self.iter().map(f).try_collect::<List<O>>()
    }

    #[doc(hidden)]
    pub fn from_elem(elem: T, n: Int) -> List<T> {
        let n = n.try_to_usize().expect("invalid number of elements in List::from_elem");
        let v: im::vector::Vector<T> = std::iter::repeat(elem).take(n).collect();

        List(GcCow::new(v))
    }
}

#[test]
fn test_list() {
    let mut l = list![1, 2, 3];
    assert_eq!(l.index_at(0), 1);
    assert_eq!(l.pop(), Some(3));
    assert_eq!(l.len(), Int::from(2));
    l.push(3);
    assert_eq!(l.len(), Int::from(3));
}
