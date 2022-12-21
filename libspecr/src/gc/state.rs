use crate::*;

pub type GcBox = Box<dyn GcCompat>;
pub struct GcState {
    data: Vec<Option<GcBox>>,

    // indices where `data` has `None` entries.
    nones: Vec<usize>,
}

impl GcState {
    pub const fn new() -> Self {
        Self {
            data: Vec::new(),
            nones: Vec::new(),
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

    pub fn capacity(&self) -> usize {
        self.data.len()
    }

    // returns bytes
    fn memory_consumption(&self) -> usize {
        // sum up the objects sizes
        self.data.iter().map(|x| x.size()).sum::<usize>()

        // each object additionally requires a fat pointer pointing to it.
        + self.capacity() * std::mem::size_of::<Box<dyn GcCompat>>()
    }

    pub fn mark_and_sweep(&mut self, roots: HashSet<usize>) {
        // don't cleanup, if you have less than 1MB allocated.
        if self.memory_consumption() < 1000 * 1000 {
            return;
        }

        // objects which are found to be reachable from `roots`, but their children were not yet added.
        let mut open = roots;

        // objects which are found to be reachable from `roots`, whose children have already been added.
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
        // seen now contains the `usize` which are reachable from roots.
        let seen = done;

        // clear all unreachable objects.
        for (i, opt) in self.data.iter_mut().enumerate() {
            if opt.is_some() && !seen.contains(&i) {
                *opt = None;
                self.nones.push(i);
            }
        }
    }
}
