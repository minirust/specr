use crate::*;

mod sparse_vec;
use sparse_vec::*;

mod internal;
mod impls;
mod trait_passthrough;

use std::marker::PhantomData;

use internal::*;

// this trait shall be implemented for each type of minirust.
// It is required in order to contain `GcCow`, and to be the generic param to `GcCow`.
pub trait GcCompat: GcCompatTrivial {
    // writes the gc'd objs, that `self` points to, into `buffer`.
    fn points_to(&self, buffer: &mut HashSet<usize>);
    fn as_any(&self) -> &dyn Any;
}

// a supertrait of GcCompat which can be automatically implemented for most types.
pub trait GcCompatTrivial: 'static {
    fn size(&self) -> usize;
}

impl<T: Sized + 'static> GcCompatTrivial for T {
    fn size(&self) -> usize { std::mem::size_of::<Self>() }
}

pub struct GcCow<T> {
    idx: usize,
    phantom: PhantomData<T>,
}

impl<T> !Send for GcCow<T> {}
impl<T> !Sync for GcCow<T> {}

impl<T> Clone for GcCow<T> {
    fn clone(&self) -> Self {
        let idx = self.idx;
        let phantom = PhantomData;
        GcCow { idx, phantom }
    }
}
impl<T> Copy for GcCow<T> {}

fn with_gc<O>(f: impl FnOnce(&GcState) -> O) -> O {
    GC_STATE.with(|st| f(&*st.borrow()))
}

fn with_gc_mut<O>(f: impl FnOnce(&mut GcState) -> O) -> O {
    GC_STATE.with(|st| f(&mut *st.borrow_mut()))
}

pub fn mark_and_sweep(roots: HashSet<usize>) {
    with_gc_mut(|st| {
        st.mark_and_sweep(roots);
    });
}

// methods for specr-internal use:
impl<T: 'static> GcCow<T> {
    pub fn new(t: T) -> Self where T: GcCompat {
        with_gc_mut(|st| {
            st.alloc(t)
        })
    }

    pub fn get(self) -> T where T: GcCompat + Clone {
        self.call_ref_unchecked(|o| o.clone())
    }

    // will fail, if `f` manipulates GC_STATE.
    pub fn call_ref_unchecked<O>(self, f: impl FnOnce(&T) -> O) -> O {
        with_gc(|st| {
            let x: &dyn Any = st.objs.get(self.idx).as_any();
            let x = x.downcast_ref::<T>().unwrap();

            f(x)
        })
    }

    // this does the copy-on-write
    pub fn mutate<O>(&mut self, f: impl FnOnce(&mut T) -> O) -> O where T: GcCompat + Clone {
        let mut val = self.get();
        let out = f(&mut val);
        *self = GcCow::new(val);

        out
    }

    // the same as above with an argument.
    // will fail, if `f` manipulates GC_STATE.
    pub fn call_ref1_unchecked<U, O>(self, arg: GcCow<U>, f: impl FnOnce(&T, &U) -> O) -> O where T: GcCompat, U: GcCompat {
        with_gc(|st| {
            let x: &dyn Any = st.objs.get(self.idx).as_any();
            let x = x.downcast_ref::<T>().unwrap();

            let arg: &dyn Any = st.objs.get(arg.idx).as_any();
            let arg = arg.downcast_ref::<U>().unwrap();

            f(x, arg)
        })
    }

    // will fail, if `f` manipulates GC_STATE.
    pub fn call_mut1_unchecked<U, O>(&mut self, arg: GcCow<U>, f: impl FnOnce(&mut T, &U) -> O) -> O where T: GcCompat + Clone, U: GcCompat {
        let mut val = self.get();
        let out = with_gc(|st| {
            let arg: &dyn Any = st.objs.get(arg.idx).as_any();
            let arg = arg.downcast_ref::<U>().unwrap();

            f(&mut val, arg)
        });

        *self = GcCow::new(val);

        out
    }
}
