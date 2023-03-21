use crate::*;
use crate::gc::*;

use std::marker::PhantomData;

/// A gargabe-collected pointer type implementing Copy.
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

impl<T: GcCompat> GcCompat for GcCow<T> {
    fn points_to(&self, buffer: &mut HashSet<usize>) {
        buffer.insert(self.idx);
    }
    fn as_any(&self) -> &dyn Any { self }
}

// methods for specr-internal use:
impl<T: GcCompat> GcCow<T> {
    /// Allocates a new `GcCow` pointing to a value `t`.
    pub fn new(t: T) -> Self where T: GcCompat {
        let idx = with_gc_mut(|st| {
            st.alloc(t)
        });
        let phantom = PhantomData;
        GcCow { idx, phantom }
    }

    /// Extracts the inner value from the `GcCow`.
    pub fn extract(self) -> T where T: GcCompat + Clone {
        self.call_ref_unchecked(|o| o.clone())
    }

    // will fail, if `f` manipulates GC_STATE.
    pub(crate) fn call_ref_unchecked<O>(self, f: impl FnOnce(&T) -> O) -> O {
        with_gc(|st| {
            let x = st.get_ref_typed::<T>(self.idx);

            f(x)
        })
    }

    // this does the copy-on-write
    pub(crate) fn mutate<O>(&mut self, f: impl FnOnce(&mut T) -> O) -> O where T: GcCompat + Clone {
        let mut val = self.extract();
        let out = f(&mut val);
        *self = GcCow::new(val);

        out
    }

    // the same as above with an argument.
    // will fail, if `f` manipulates GC_STATE.
    pub(crate) fn call_ref1_unchecked<U, O>(self, arg: GcCow<U>, f: impl FnOnce(&T, &U) -> O) -> O where T: GcCompat, U: GcCompat {
        with_gc(|st| {
            let x = st.get_ref_typed::<T>(self.idx);
            let arg = st.get_ref_typed::<U>(arg.idx);

            f(x, arg)
        })
    }

    // will fail, if `f` manipulates GC_STATE.
    pub(crate) fn call_mut1_unchecked<U, O>(&mut self, arg: GcCow<U>, f: impl FnOnce(&mut T, &U) -> O) -> O where T: GcCompat + Clone, U: GcCompat {
        let mut val = self.extract();
        let out = with_gc(|st| {
            let arg = st.get_ref_typed::<U>(arg.idx);

            f(&mut val, arg)
        });

        *self = GcCow::new(val);

        out
    }
}
