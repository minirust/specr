use crate::*;
use crate::gc::*;

use std::marker::PhantomData;

/// A gargabe-collected pointer type implementing Copy.
pub struct GcCow<T> {
    idx: usize,
    phantom: PhantomData<T>,
}

// Keep them in the same thread, since the GC state is per-thread.
impl<T> !Send for GcCow<T> {}
impl<T> !Sync for GcCow<T> {}

// Need a custom impl to avoid the `T: Clone` bound.
impl<T> Clone for GcCow<T> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<T> Copy for GcCow<T> {}

impl<T: 'static> GcCompat for GcCow<T> {
    fn points_to(&self, buffer: &mut HashSet<usize>) {
        // We do *not* recurse into the pointee, that is the job of mark-and-sweep!
        buffer.insert(self.idx);
    }
}

// methods for specr-internal use:
impl<T: GcCompat> GcCow<T> {
    /// Allocates a new `GcCow` pointing to a value `t`.
    pub fn new(t: T) -> Self {
        let idx = with_gc_mut(|st| {
            st.alloc(t)
        });
        let phantom = PhantomData;
        GcCow { idx, phantom }
    }

    /// Extracts the inner value from the `GcCow`.
    pub fn extract(self) -> T where T: Clone {
        self.call_ref_unchecked(|o| o.clone())
    }

    /// will fail, if `f` manipulates GC_STATE.
    pub(crate) fn call_ref_unchecked<O>(self, f: impl FnOnce(&T) -> O) -> O {
        with_gc(|st| {
            let x = st.get_ref_typed::<T>(self.idx);

            f(x)
        })
    }

    /// this does the clone-on-write
    pub(crate) fn mutate<O>(&mut self, f: impl FnOnce(&mut T) -> O) -> O where T: Clone {
        let mut val = self.extract(); // get a clone
        let out = f(&mut val); // act on the clone
        *self = GcCow::new(val); // put the result into the GC heap

        out
    }

    /// The same as `call_ref_unchecked`, but gives access to two `GcCow`.
    /// will fail, if `f` manipulates GC_STATE.
    pub(crate) fn call_ref1_unchecked<U: GcCompat, O>(self, arg: GcCow<U>, f: impl FnOnce(&T, &U) -> O) -> O {
        with_gc(|st| {
            let x = st.get_ref_typed::<T>(self.idx);
            let arg = st.get_ref_typed::<U>(arg.idx);

            f(x, arg)
        })
    }

    /// The same as `mutate` but also gives read-only access to another `GcCow`.
    /// will fail, if `f` manipulates GC_STATE.
    pub(crate) fn call_mut1_unchecked<U: GcCompat, O>(&mut self, arg: GcCow<U>, f: impl FnOnce(&mut T, &U) -> O) -> O where T: Clone {
        let mut val = self.extract();
        let out = with_gc(|st| {
            let arg = st.get_ref_typed::<U>(arg.idx);

            f(&mut val, arg)
        });

        *self = GcCow::new(val);

        out
    }
}
