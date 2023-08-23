use crate::*;

use std::ops::*;

impl<T: Obj> List<T> {
    /// Returns an empty `List`.
    pub fn new() -> Self {
        Self(Default::default())
    }

    /// Returns the number of elements in `self`.
    pub fn len(&self) -> Int {
        Int::from(self.0.call_ref_unchecked(|v| v.len()))
    }

    /// Returns `true` if the list contains no elements.
    pub fn is_empty(self) -> bool {
        self.0.call_ref_unchecked(|v| v.is_empty())
    }

    /// Returns the first element of `self`
    /// or `None` if `self` is empty.
    pub fn first(&self) -> Option<T> {
        self.0.call_ref_unchecked(|v| v.front().cloned())
    }

    /// Returns the last element of `self`
    /// or `None` if `self` is empty.
    pub fn last(&self) -> Option<T> {
        self.0.call_ref_unchecked(|v| v.last().cloned())
    }

    /// Conceptually equivalent to `f(&mut self[i])`.
    /// Mutates the `i`th element by giving a mutable ref of it into the function `f`.
    /// The return value of `f` will be returned from `mutate_at`.
    ///
    /// Instead of actual mutation, a new list is allocated, where only the `i`th element is changed.
    /// Then `self` is changed so that it points to that new list.
    pub fn mutate_at<O: Obj>(&mut self, i: Int, f: impl FnOnce(&mut T) -> O) -> O {
        let i = i.try_to_usize().expect("List::mutate_at: index out of range of `usize`!");
        self.0.mutate(|v| f(&mut v[i]))
    }
    /// Like `mutate_at`, but the closure is fallible
    pub fn try_mutate_at<O: Obj, E>(&mut self, i: Int, f: impl FnOnce(&mut T) -> NdResult<O, E>) -> NdResult<O, E> {
        let i = i.try_to_usize().expect("List::mutate_at: index out of range of `usize`!");
        self.0.mutate(|v| f(&mut v[i]))
    }

    /// Returns the `i`th element of the list.
    pub fn get(&self, i: Int) -> Option<T> {
        let i = i.try_to_usize().expect("List::get: index out of range of `usize`!");
        self.0.call_ref_unchecked(|v| v.get(i).cloned())
    }

    /// Sets the `i`th element of the list.
    pub fn set(&mut self, i: Int, t: T) {
        self.mutate_at(i, |r| { *r = t; } )
    }

    /// The indexing operator:
    /// specr translates `a[b]` to `a.index_at(b)`.
    pub fn index_at(&self, i: impl Into<Int>) -> T {
        self.get(i.into()).unwrap()
    }

    /// Push an element to the end of the list.
    pub fn push(&mut self, t: T) {
        self.0.mutate(|v| v.push_back(t));
    }

    /// Pop the element from the end of the list.
    /// Returns `None` is the list was empty.
    pub fn pop(&mut self) -> Option<T> {
        self.0.mutate(|v| v.pop_back())
    }

    /// Pop the element from the beginning of the list.
    /// Returns `None` is the list was empty.
    pub fn pop_front(&mut self) -> Option<T> {
        self.0.mutate(|v| v.pop_front())
    }

    /// Reverse the list.
    pub fn reverse(&mut self) {
        let v: IMVector<T> = self.0.call_ref_unchecked(|v| v.iter().cloned().rev().collect());
        *self = List(GcCow::new(v));
    }

    /// Conceptually equivalent to `self[start..length]`.
    pub fn subslice_with_length(&self, start: Int, length: Int) -> List<T> {
        let start = start.try_to_usize().expect("List::subslice_with_length: `start` out of range of `usize`!");
        let length = length.try_to_usize().expect("List::subslice_with_length: `length` out of range of `usize`!");

        // exclusive end
        let end = start + length;

        let v: IMVector<T> = self.0.call_ref_unchecked(|v| v.clone().slice(start..end));

        List(GcCow::new(v))
    }

    /// Conceptually equivalent to `self[start..src.len()] = src;`
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

    /// Sorts the list with a key extraction function.
    // note that `f` could modify the GC_STATE.
    pub fn sort_by_key<K: Obj + Ord>(&mut self, mut f: impl FnMut(T) -> K) {
        // I think we don't lose too much performance by going to a Vec here, as
        // (1.) it's only used once at all, for tuple types to order their fields.
        // (2.) At least in the case of doing actual notable amounts of sorting `im` needs to fully re-create the vec anyways.
        let mut vec: Vec<T> = self.iter().collect();
        vec.sort_by_key(|t| f(*t));
        *self = vec.into_iter().collect();
    }

    /// Combine two lists to a list of pairs.
    pub fn zip<T2: Obj>(self, other: List<T2>) -> List<(T, T2)> {
        self.iter().zip(other.iter()).collect()
    }

    /// Tests if any element of the list matches a predicate.
    pub fn any(self, f: impl FnMut(T) -> bool) -> bool {
        self.iter().any(f)
    }

    /// Tests if all elements of the list matches a predicate.
    pub fn all(self, f: impl FnMut(T) -> bool) -> bool {
        self.iter().all(f)
    }

    /// applies `f` to each element of the list and returns the outputs as another list.
    pub fn map<O: Obj>(self, f: impl FnMut(T) -> O) -> List<O> {
        self.iter().map(f).collect()
    }

    /// Works like map, but flattens nested structure.
    pub fn flat_map<O: Obj>(self, f: impl FnMut(T) -> List<O>) -> List<O> {
        self.iter().flat_map(f).collect()
    }

    /// Applies `f` to each element of the list and returns the successful outputs as another lists.
    /// If at least one call to `f` failed its error is returned instead.
    pub fn try_map<O: Obj, E>(self, f: impl FnMut(T) -> E) -> <<E as Try>::Residual as Residual<List<O>>>::TryType
        where E: Try<Output=O>,
              <E as Try>::Residual: Residual<List<O>>,
    {
        self.iter().map(f).try_collect::<List<O>>()
    }

    // This is required for the list![a; n] macro.
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
