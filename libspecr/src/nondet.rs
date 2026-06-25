use crate::*;

use std::ops::*;

#[derive(Copy, Clone, GcCompat)]
/// Non-determinism primitive. See [Non-determinism](https://github.com/minirust/minirust/blob/master/README.md#non-determinism).
pub struct Nondet<T>(pub(crate) T);

/// The `pick` function from the minirust spec.  See [Non-determinism](https://github.com/minirust/minirust/blob/master/README.md#non-determinism).
pub fn pick<T: Obj>(distr: impl Distribution<T>, f: impl Fn(T) -> bool) -> crate::Nondet<T> {
    let mut rng = rand::thread_rng();
    for _ in 0..50 {
        let s = distr.sample(&mut rng);
        if f(s) {
            return Nondet(s);
        }
    }

    panic!("Timeout! `pick` could not find a valid value.");
}


/// The `Try::Residual` for `Nondet`. This type is empty because `Nondet` has no residual
/// as using the `?` operator on a `Nondet<T>` can not fail. We can not use `Infallible` here
/// since we must implement the `Residual` trait, which coherence blocks us from doing for
/// foreign types.
pub enum NondetResidual {}


/// The `predict` function from the minirust spec. See [Non-determinism](https://github.com/minirust/minirust/blob/master/README.md#non-determinism).
pub fn predict<T>(_f: impl Fn(T) -> bool) -> crate::Nondet<T> { unimplemented!() }

impl<T> Try for Nondet<T> {
    type Output = T;
    type Residual = NondetResidual;

    fn from_output(output: Self::Output) -> Self {
        Nondet(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
       ControlFlow::Continue(self.0)
    }
}

impl<T> FromResidual<NondetResidual> for Nondet<T> {
    fn from_residual(residual: NondetResidual) -> Self {
        match residual {}
    }
}

impl<T> Residual<T> for NondetResidual {
    type TryType = Nondet<T>;
}
