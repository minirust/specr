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

pub(crate) mod unnameable_infallible {

    /// An empty type that we can implement foreign traits on without violating coherence rules.
    /// Used because `Try` requires that its `Residual` implements a certain trait.
    /// This type is not `pub`-licly nameable directly due to the surrounding module,
    /// but still `pub` since it can be referred to as `Nondet<T>::Residual` due to the trait implementation below.
    pub enum MyInfallible {}

}

use unnameable_infallible::MyInfallible;

/// The `predict` function from the minirust spec. See [Non-determinism](https://github.com/minirust/minirust/blob/master/README.md#non-determinism).
pub fn predict<T>(_f: impl Fn(T) -> bool) -> crate::Nondet<T> { unimplemented!() }

impl<T> Try for Nondet<T> {
    type Output = T;
    type Residual = MyInfallible;

    fn from_output(output: Self::Output) -> Self {
        Nondet(output)
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
       ControlFlow::Continue(self.0)
    }
}

impl<T> FromResidual<MyInfallible> for Nondet<T> {
    fn from_residual(residual: MyInfallible) -> Self {
        match residual {}
    }
}

impl<T> Residual<T> for MyInfallible {
    type TryType = Nondet<T>;
}
