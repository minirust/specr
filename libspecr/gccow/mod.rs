mod internal;
mod sparse_vec;
mod impls;

use std::collections::HashSet;
use std::any::Any;
use std::marker::PhantomData;
use std::fmt::{Formatter, Debug, Error};
use std::hash::{Hash, Hasher};

use internal::*;

// this trait shall be implemented for each type of minirust.
// It is required in order to contain `GcCow`, and to be the generic param to `GcCow`.
pub trait GcCompat {
    // writes the gc'd objs, that `self` points to, into `buffer`.
    fn points_to(&self, buffer: &mut HashSet<usize>);
    fn as_any(&self) -> &dyn Any;
}

pub struct GcCow<T> {
    idx: usize,
    phantom: PhantomData<T>,
}

impl<T> Clone for GcCow<T> {
    fn clone(&self) -> Self {
        let idx = self.idx;
        let phantom = PhantomData;
        GcCow { idx, phantom }
    }
}
impl<T> Copy for GcCow<T> {}

// those are free functions instead of GcCow methods, so that they can be individually included in the hidden module.
pub fn gccow_new<T>(t: T) -> GcCow<T> where T: GcCompat {
    GC_STATE.with(|st| {
        st.borrow_mut().alloc(t)
    })
}

pub fn gccow_get<T>(gc: &GcCow<T>) -> T where T: GcCompat + Copy {
    GC_STATE.with(|st| {
        let st: &GcState = &*st.borrow();
        let x: &dyn Any = st.objs.get(gc.idx).as_any();
        let r = x.downcast_ref::<T>().unwrap();

        r.clone()
    })
}

pub fn gccow_mutate<T>(gc: &mut GcCow<T>, f: impl Fn(&mut T)) where T: GcCompat + Copy {
    gc.call_mut(f);
}

pub fn mark_and_sweep(roots: HashSet<usize>) {
    GC_STATE.with(|st| {
        let mut st = st.borrow_mut();
        st.mark_and_sweep(roots);
    });
}

// methods for specr-internal use:
impl<T> GcCow<T> {
    pub(in crate::specr) fn call_ref<O>(self, f: impl Fn(&T) -> O) -> O {
        GC_STATE.with(|st| {
            let st: &GcState = &*st.borrow();
            let x: &dyn Any = st.objs.get(self.idx).as_any();
            let x = x.downcast_ref::<T>().unwrap();

            f(x)
        })
    }
}

impl<T> GcCow<T> {
    // this does the copy-on-write
    pub(in crate::specr) fn call_mut<O>(&mut self, f: impl Fn(&mut T) -> O) -> O where T: GcCompat {
        let mut val = gccow_get(self);
        let out = f(&mut val);
        *self = gccow_new(val);

        out
    }
}

// the same as above with an argument.
impl<T> GcCow<T> {
    pub(in crate::specr) fn call_ref1<U, O>(self, arg: GcCow<U>, f: impl Fn(&T, &U) -> O) -> O {
        GC_STATE.with(|st| {
            let st: &GcState = &*st.borrow();
            let x: &dyn Any = st.objs.get(self.idx).as_any();
            let x = x.downcast_ref::<T>().unwrap();

            let arg: &dyn Any = st.objs.get(arg.idx).as_any();
            let arg = x.downcast_ref::<T>().unwrap();

            f(x, arg)
        })
    }
}

impl<T> GcCow<T> {
    pub(in crate::specr) fn call_mut1<U, O>(&mut self, arg: GcCow<U>, f: impl Fn(&mut T, &U) -> O) -> O where T: GcCompat {
        let mut val = gccow_get(self);
        let out = GC_STATE.with(|st| {
            let st: &GcState = &*st.borrow();

            let x: &dyn Any = st.objs.get(arg.idx).as_any();
            let x = x.downcast_ref::<T>().unwrap();

            f(&mut val, x)
        });
        *self = gccow_new(val);

        out
    }
}

impl<T> Debug for GcCow<T> where T: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        self.call_ref(|t| t.fmt(f))
    }
}

impl<T> Hash for GcCow<T> where T: Hash {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.call_ref(|t| t.hash(state))
    }
}

impl<T> PartialEq for GcCow<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.call_ref1(*other, |s, o| s == o)
    }
}

impl<T> Eq for GcCow<T> where T: Eq {}
