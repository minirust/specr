use crate::*;

use std::convert::Infallible;
use std::ops::*;

#[derive(Copy, Clone)]
/// Non-determinism primitive. See [Non-determinism](https://github.com/RalfJung/minirust/blob/master/README.md#non-determinism).
pub struct Nondet<T>(pub T);

/// A probability distribution over values of type `T`.
pub trait Distribution<T> {
    /// samples a value from the distribution.
    fn sample(&self, rng: &mut ThreadRng) -> T;
}

impl Distribution<Int> for Range<Int> {
    fn sample(&self, rng: &mut ThreadRng) -> Int {
        let start = self.start.ext();
        let end = self.end.ext();

        assert!(start < end);
        let range = (end - &start).to_biguint().unwrap();

        // we first generate a random number `ext` in 0..range
        // we use `to_bytes_be` to get the number of bytes required to store a number in 0..range.
        let mut bytes = range.to_bytes_be();
        rng.fill_bytes(&mut bytes);
        // This number might be `>= range` still.
        let uint: ExtUint = ExtUint::from_bytes_be(&bytes);
        let uint: ExtUint = uint % range;
        let ext: ExtInt = uint.into();

        // `out` in start..end, because
        // `ext` in 0..range
        let out = ext + start;

        Int::wrap(out)
    }
}

/// The `pick` function from the minirust spec.  See [Non-determinism](https://github.com/RalfJung/minirust/blob/master/README.md#non-determinism).
pub fn pick<T: Obj>(distr: impl Distribution<T>, f: impl Fn(T) -> bool) -> crate::Nondet<T> {
    let mut rng = thread_rng();
    for _ in 0..50 {
        let s = distr.sample(&mut rng);
        if f(s) {
            return Nondet(s);
        }
    }

    panic!("Timeout! `pick` could not find a valid value.");
}

/// The `predict` function from the minirust spec. See [Non-determinism](https://github.com/RalfJung/minirust/blob/master/README.md#non-determinism).
pub fn predict<T>(_f: impl Fn(T) -> bool) -> crate::Nondet<T> { unimplemented!() }

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
