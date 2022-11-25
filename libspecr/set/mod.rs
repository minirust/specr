use crate::libspecr::*;

mod func;
mod iter;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Set<T: Obj>(pub(in crate::libspecr) GcCow<IMHashSet<T>>);

impl<T: Obj> GcCompat for Set<T> {
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl<T: Obj> GcCompat for IMHashSet<T> {
    fn points_to(&self, m: &mut HashSet<usize>) {
        for x in self.iter() {
            x.points_to(m);
        }
    }
    fn as_any(&self) -> &dyn Any { self }
}

// This is not #[derive]d, as this would wrongly require T: Default.
impl<T: Obj> Default for Set<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}
