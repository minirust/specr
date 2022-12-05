use crate::*;

use std::convert::Infallible;
use std::ops::*;

#[derive(Copy, Clone)]
/// Non-determinism primitive. See [Non-determinism](https://github.com/RalfJung/minirust/blob/master/README.md#non-determinism).
pub struct Nondet<T>(pub T);

/// The `pick` function from the minirust spec.  See [Non-determinism](https://github.com/RalfJung/minirust/blob/master/README.md#non-determinism).
pub fn pick<T>(_f: impl Fn(T) -> bool) -> crate::Nondet<T> { todo!() }

/// The `predict` function from the minirust spec. See [Non-determinism](https://github.com/RalfJung/minirust/blob/master/README.md#non-determinism).
pub fn predict<T>(_f: impl Fn(T) -> bool) -> crate::Nondet<T> { todo!() }

impl<T: GcCompat> GcCompat for Nondet<T> {
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl<T> Try for Nondet<T> {
    type Output = T;
    type Residual = Infallible;

    fn from_output(output: Self::Output) -> Self {
        Nondet(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
       ControlFlow::Continue(self.0)
    }
}

impl<T> FromResidual<Infallible> for Nondet<T> {
    fn from_residual(residual: Infallible) -> Self {
        match residual {}
    }
}
