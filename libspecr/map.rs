use crate::libspecr::*;

pub struct Map<K, V>(GcCow<IMHashMap<K, V>>);

impl<K, V> Clone for Map<K, V> {
    fn clone(&self) -> Self { Map(self.0) }
}
impl<K, V> Copy for Map<K, V> {}

impl<K, V> GcCompat for Map<K, V> where K: GcCompat, V: GcCompat {
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl<K, V> GcCompat for IMHashMap<K, V> where K: GcCompat, V: GcCompat {
    fn points_to(&self, m: &mut HashSet<usize>) {
        for (k, v) in self.iter() {
            k.points_to(m);
            v.points_to(m);
        }
    }
    fn as_any(&self) -> &dyn Any { self }
}
