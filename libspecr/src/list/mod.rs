use crate::*;

mod iter;
mod func;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct List<T: Obj>(pub GcCow<IMVector<T>>);

pub macro list {
	() => { List::new() },
	($start:expr $(,$a:expr)*) => { [$start $(,$a)* ].into_iter().collect::<List<_>>() },
	($a:expr ; $b:expr) => { list_from_elem($a, Int::from($b)) },
}

impl<T: Obj> GcCompat for IMVector<T> {
    fn points_to(&self, m: &mut HashSet<usize>) {
        for i in self.iter() {
            i.points_to(m);
        }
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl<T: Obj> GcCompat for List<T> {
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}

// This is not #[derive]d, as this would wrongly require T: Default.
impl<T: Obj> Default for List<T> {
    fn default() -> Self {
        Self(Default::default())
    }
}

