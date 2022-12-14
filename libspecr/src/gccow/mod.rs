use crate::*;

mod sparse_vec;
use sparse_vec::*;

mod internal;
mod impls;
mod trait_passthrough;

use std::marker::PhantomData;
use std::sync::Mutex;

use internal::*;

// this trait shall be implemented for each type of minirust.
// It is required in order to contain `GcCow`, and to be the generic param to `GcCow`.
pub trait GcCompat: Send + Sync + 'static {
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

fn with_gc<O>(f: impl FnOnce(&GcState) -> O) -> O {
    let st = GC_STATE.try_read().unwrap();
    f(&*st)
}

fn with_gc_mut<O>(f: impl FnOnce(&mut GcState) -> O) -> O {
    let mut st = GC_STATE.try_write().unwrap();
    f(&mut *st)
}

pub static SEQUENTIAL_LOCK: Mutex<()> = Mutex::new(());

/// This function is used to syncronize tests.
///
/// Per default, tests run in parallel, which might cause tests to `mark_and_sweep` the other tests' objects.
/// Hence tests should be run inside of this function.
pub fn run_sequential(f: impl FnOnce()) {
    let _lock = match SEQUENTIAL_LOCK.lock() {
        Ok(x) => x,
        Err(x) => x.into_inner(), // ignore poison.
    };

    // pre cleanup for the case that someone poisoned `SEQUENTIAL_LOCK` in an uncleared state.
    mark_and_sweep(HashSet::new());

    f();

    // post cleanup
    mark_and_sweep(HashSet::new());
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
