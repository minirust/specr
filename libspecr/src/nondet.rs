use crate::*;

#[derive(Copy, Clone)]
/// Non-determinism primitive. See [Non-determinism](https://github.com/RalfJung/minirust/blob/master/README.md#non-determinism).
pub struct Nondet<T>(pub T);

// TODO this is probably redundant.
/// Wrapper around `do yeet` expressions.
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
