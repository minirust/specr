use crate::*;

#[derive(Copy, Clone)]
pub struct Nondet<T>(pub T);

pub macro yeet {
    ($x:expr) => {
        do yeet $x
    },
}

impl<T: GcCompat> GcCompat for Nondet<T> {
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}
