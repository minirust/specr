use crate::{*, gccow::*};

use std::marker::PhantomData;
use std::cell::RefCell;

impl<T: GcCompat> GcCompat for GcCow<T> {
    fn points_to(&self, buffer: &mut HashSet<usize>) {
        buffer.insert(self.idx);
    }
    fn as_any(&self) -> &dyn Any { self }
}

pub(crate) struct GcState {
    pub objs: SparseVec<Box<dyn GcCompat>>,
}

// Note that each thread has it's own GC_STATE.
// You cannot share/send Garbage collected objects over threads.
thread_local! {
    pub(crate) static GC_STATE: RefCell<GcState> = RefCell::new(GcState::new());
}

impl GcState {
    pub const fn new() -> GcState {
        Self { objs: SparseVec::new() }
    }

    pub fn alloc<T: GcCompat>(&mut self, t: T) -> GcCow<T> {
        let obj: Box<dyn GcCompat> = Box::new(t);
        let idx = self.objs.insert(obj);
        GcCow { idx, phantom: PhantomData }
    }

    // returns bytes
    fn memory_consumption(&self) -> usize {
        // sum up the objects sizes
        self.objs.iter().map(|x| x.size()).sum::<usize>()

        // each object additionally requires a fat pointer pointing to it.
        + self.objs.capacity() * std::mem::size_of::<Box<dyn GcCompat>>()
    }

    pub fn mark_and_sweep(&mut self, roots: HashSet<usize>) {
        // don't cleanup, if you have less than 1MB allocated.
        if self.memory_consumption() < 1000 * 1000 {
            return;
        }

        // `objs` which are found to be reachable from `roots`, but their children were not yet added.
        let mut open = roots;

        // `objs` which are found to be reachable from `roots`, whose children have already been added.
        let mut done = HashSet::new();

        while let Some(i) = open.iter().next().cloned() {
            open.remove(&i);
            done.insert(i);

            let mut current = HashSet::new();
            self.objs.get(i).points_to(&mut current);

            for new in current {
                if !done.contains(&new) && !open.contains(&new) {
                    open.insert(new);
                }
            }
        }
        // seen now contains the `usize` which are reachable from roots.
        let seen = done;

        self.objs.retain(seen);
    }
}
