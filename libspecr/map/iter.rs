use crate::libspecr::*;

// TODO this is merely a temporary solution.
// this iterator is not lazy!
struct MapIter<K, V> {
    data: Vec<(K, V)>,
    idx: usize,
}

impl<K, V> Iterator for MapIter<K, V> where K: Clone, V: Clone {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        let x = self.data.get(self.idx)?;
        self.idx += 1;

        Some(x.clone())
    }
}


impl<K, V> Map<K, V> where K: GcCompat + Clone + Hash + Eq, V: GcCompat + Clone {
    pub fn iter(self) -> MapIter<K, V> {
        let map = self.0.get();
        let data = map.iter().map(|(x, y)| (x.clone(), y.clone())).collect();
        MapIter { data, idx: 0 }
    }
}

impl<K, V> IntoIterator for Map<K, V> where K: GcCompat + Clone + Hash + Eq, V: GcCompat + Clone {
    type Item = (K, V);
    type IntoIter = MapIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
