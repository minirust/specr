mod internal;
mod sparse_vec;
mod impls;

use std::collections::HashSet;
use std::any::Any;
use std::marker::PhantomData;

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

// this does the copy-on-write
pub fn gccow_mutate<T>(gc: &mut GcCow<T>, f: impl Fn(&mut T)) where T: GcCompat + Copy {
    let mut val = gccow_get(gc);
    f(&mut val);
    *gc = gccow_new(val);
}

pub fn mark_and_sweep(roots: HashSet<usize>) {
    GC_STATE.with(|st| {
        let mut st = st.borrow_mut();
        st.mark_and_sweep(roots);
    });
}
