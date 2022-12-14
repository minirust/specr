use crate::*;

pub struct SparseVec<T> {
    data: Vec<Option<T>>,
    // indices with `None` entries.
    nones: Vec<usize>,
}

impl<T> SparseVec<T> {
    pub const fn new() -> SparseVec<T> {
        SparseVec {
            data: Vec::new(),
            nones: Vec::new(),
        }
    }

    pub fn insert(&mut self, t: T) -> usize {
        let idx = match self.nones.pop() {
            Some(i) => i,
            None => {
                let i = self.data.len();
                self.data.push(None);
                i
            }
        };

        self.data[idx] = Some(t);
        idx
    }

    pub fn get(&self, i: usize) -> &T {
        self.data[i].as_ref().expect("invalid get")
    }

    pub fn retain(&mut self, seen: HashSet<usize>) {
        for (i, opt) in self.data.iter_mut().enumerate() {
            if opt.is_some() && !seen.contains(&i) {
                *opt = None;
                self.nones.push(i);
            }
        }
    }

    #[allow(unused)]
    pub fn len(&self) -> usize {
        self.data.len() - self.nones.len()
    }
}
