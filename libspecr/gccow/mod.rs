use crate::libspecr::*;

mod sparse_vec;
use sparse_vec::*;

mod internal;
mod impls;

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

pub fn mark_and_sweep(roots: HashSet<usize>) {
    GC_STATE.with(|st| {
        let mut st = st.borrow_mut();
        st.mark_and_sweep(roots);
    });
}

// methods for specr-internal use:
impl<T> GcCow<T> {
    pub fn new(t: T) -> Self where T: GcCompat {
        GC_STATE.with(|st| {
            st.borrow_mut().alloc(t)
        })
    }

    pub fn get(self) -> T where T: GcCompat + Clone {
        self.call_ref(|o| o.clone())
    }

    // TODO this fn and it's variants might cause RefCell problems, if `f` does eg. GcCow::new().
    pub fn call_ref<O>(self, f: impl Fn(&T) -> O) -> O {
        GC_STATE.with(|st| {
            let st: &GcState = &*st.borrow();
            let x: &dyn Any = st.objs.get(self.idx).as_any();
            let x = x.downcast_ref::<T>().unwrap();

            f(x)
        })
    }

    // this does the copy-on-write
    pub fn call_mut<O>(&mut self, f: impl Fn(&mut T) -> O) -> O where T: GcCompat + Clone {
        let mut val = self.get();
        let out = f(&mut val);
        *self = GcCow::new(val);

        out
    }

    // the same as above with an argument.
    pub fn call_ref1<U, O>(self, arg: GcCow<U>, f: impl Fn(&T, &U) -> O) -> O where T: GcCompat, U: GcCompat {
        GC_STATE.with(|st| {
            let st: &GcState = &*st.borrow();
            let x: &dyn Any = st.objs.get(self.idx).as_any();
            let x = x.downcast_ref::<T>().unwrap();

            let arg: &dyn Any = st.objs.get(arg.idx).as_any();
            let arg = arg.downcast_ref::<U>().unwrap();

            f(x, arg)
        })
    }

    pub fn call_mut1<U, O>(&mut self, arg: GcCow<U>, f: impl Fn(&mut T, &U) -> O) -> O where T: GcCompat + Clone, U: GcCompat {
        let mut val = self.get();
        let out = GC_STATE.with(|st| {
            let st: &GcState = &*st.borrow();

            let arg: &dyn Any = st.objs.get(arg.idx).as_any();
            let arg = arg.downcast_ref::<U>().unwrap();

            f(&mut val, arg)
        });

        *self = GcCow::new(val);

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

impl<T> PartialEq for GcCow<T> where T: GcCompat + PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.call_ref1(*other, |s, o| s == o)
    }
}

impl<T> Eq for GcCow<T> where T: GcCompat + Eq {}
