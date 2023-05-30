use std::mem;
use crate::*;

// mark_and_sweep won't cleanup, if you didn't allocate at least LEEWAY_MEMORY bytes since the last cleanup.
const LEEWAY_MEMORY : usize = 1000 * 1000; // 1MB

pub type GcBox = Box<dyn GcCompat>;
pub struct GcState {
    data: Vec<Option<GcBox>>,

    // indices where `data` has `None` entries.
    nones: Vec<usize>,

    // How much memory is used right now (in bytes)
    // We don't count std::mem::size_of::<GcState>, nor the heap-allocated data directly behind `data` or `nones`.
    // because those things are not reduced when doing mark and sweep.
    // We only count the size within the `GcBox`es.
    current_memory: usize,

    // How much memory was used right after the last mark and sweep (in bytes)
    last_memory: usize,
}

impl GcState {
    pub const fn new() -> Self {
        Self {
            data: Vec::new(),
            nones: Vec::new(),
            current_memory: 0,
            last_memory: 0,
        }
    }

    pub fn clear(&mut self) {
        self.data.clear();
        self.nones.clear();
    }

    pub fn alloc<T: GcCompat>(&mut self, t: T) -> usize {
        let obj: Box<dyn GcCompat> = Box::new(t);
        let idx = match self.nones.pop() {
            Some(i) => i,
            None => {
                let i = self.data.len();
                self.data.push(None);
                i
            }
        };

        self.current_memory += mem::size_of_val(&*obj);
        self.data[idx] = Some(obj);

        idx
    }

    fn get_ref(&self, i: usize) -> &dyn GcCompat {
        let x: &Box<dyn GcCompat> = self.data[i].as_ref().expect("invalid get");

        (*x).as_ref()
    }

    pub fn get_ref_typed<T: GcCompat>(&self, i: usize) -> &T {
        let x: &dyn Any = self.get_ref(i).as_any();
        let x: &T = x.downcast_ref::<T>().expect("type error");

        x
    }

    #[allow(unused)]
    pub fn len(&self) -> usize {
        self.data.len() - self.nones.len()
    }

    #[allow(unused)]
    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    fn full_memory_consumption(&self) -> usize {
        self.data.iter().map(|x| mem::size_of_val(&*x)).sum::<usize>()
    }

    pub fn mark_and_sweep(&mut self, root: &impl GcCompat) {
        // don't cleanup, if you didn't allocate at least LEEWAY_MEMORY bytes since the last cleanup.
        if self.current_memory < self.last_memory + LEEWAY_MEMORY {
            return;
        }

        // objects which are found to be directly reachable from `root`.
        let mut open = HashSet::new();
        root.points_to(&mut open);

        // objects which are found to be reachable from `root`, whose children have already been added.
        let mut done = HashSet::new();

        while let Some(i) = open.iter().next().cloned() {
            open.remove(&i);
            done.insert(i);

            let mut current = HashSet::new();
            self.get_ref(i).points_to(&mut current);

            for new in current {
                if !done.contains(&new) && !open.contains(&new) {
                    open.insert(new);
                }
            }
        }
        // seen now contains the `usize` which are reachable from root.
        let seen = done;

        // clear all unreachable objects.
        for (i, opt) in self.data.iter_mut().enumerate() {
            if opt.is_some() && !seen.contains(&i) {
                *opt = None;
                self.nones.push(i);
            }
        }

        self.current_memory = self.full_memory_consumption();
        self.last_memory = self.current_memory;
    }
}
