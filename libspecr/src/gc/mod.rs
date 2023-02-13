use crate::*;

mod gccompat;
pub use gccompat::*;

mod state;
pub use state::*;

mod gccow;
pub use gccow::*;

mod trait_passthrough;

use std::cell::RefCell;

// Note that each thread has it's own GC_STATE.
// You cannot share/send Garbage collected objects over threads.
thread_local! {
    pub(crate) static GC_STATE: RefCell<GcState> = RefCell::new(GcState::new());
}

fn with_gc<O>(f: impl FnOnce(&GcState) -> O) -> O {
    GC_STATE.with(|st| f(&*st.borrow()))
}

fn with_gc_mut<O>(f: impl FnOnce(&mut GcState) -> O) -> O {
    GC_STATE.with(|st| f(&mut *st.borrow_mut()))
}

/// clears every object not recursively reachable from `roots`.
pub fn mark_and_sweep(roots: HashSet<usize>) {
    with_gc_mut(|st| st.mark_and_sweep(roots) );
}

/// clears all objects from the garbage collector.
pub fn clear() {
    with_gc_mut(|st| st.clear() );
}
