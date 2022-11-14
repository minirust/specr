mod internal;
mod sparse_vec;
mod impls;

use std::collections::HashSet;
use std::any::Any;
use std::marker::PhantomData;

use internal::*;

// this trait shall be implemented for each type of minirust.
// It is required in order to contain `GcCow`, and to be the generic param to `GcCow`.
pub trait GcCompat: Send + Sync + 'static {
    // writes the things `self` points to into `buffer`.
    fn points_to(&self, buffer: &mut HashSet<usize>);
    fn as_any(&self) -> &dyn Any;
}

#[derive(Copy, Clone)]
pub struct GcCow<T: GcCompat> {
    idx: usize,
    phantom: PhantomData<T>,
}

impl<T: GcCompat> GcCow<T> {
    pub fn new(t: T) -> GcCow<T> {
        GC_STATE.with(|st| {
            st.borrow_mut().alloc(t)
        })
    }

    pub fn get(&self) -> T where T: Copy {
        GC_STATE.with(|st| {
            let st: &GcState = &*st.borrow();
            let x: &dyn Any = st.objs.get(self.idx).as_any();
            let r = x.downcast_ref::<T>().unwrap();

            r.clone()
        })
    }

    // this does the copy-on-write
    pub fn mutate(&mut self, f: impl Fn(&mut T)) where T: Copy {
        let mut val = self.get();
        f(&mut val);
        *self = GcCow::new(val);
    }
}

pub fn mark_and_sweep(roots: HashSet<usize>) {
    GC_STATE.with(|st| {
        let mut st = st.borrow_mut();
        st.mark_and_sweep(roots);
    });
}
