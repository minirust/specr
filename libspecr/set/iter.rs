use crate::libspecr::*;

// TODO this is merely a temporary solution.
// this iterator is not lazy!
struct SetIter<T> {
    data: Vec<T>,
    idx: usize,
}

impl<T> Iterator for SetIter<T> where T: Clone {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.data.get(self.idx)?;
        self.idx += 1;

        Some(x.clone())
    }
}


impl<T> Set<T> where T: GcCompat + Clone + Hash + Eq {
    pub fn iter(self) -> SetIter<T> {
        let set = self.0.get();
        let data = set.iter().cloned().collect();
        SetIter { data, idx: 0 }
    }
}

impl<T> IntoIterator for Set<T> where T: GcCompat + Clone + Hash + Eq {
    type Item = T;
    type IntoIter = SetIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
