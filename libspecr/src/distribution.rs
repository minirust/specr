use crate::*;

use rand::{rngs::ThreadRng, Rng};
use num_bigint::RandBigInt;
use num_integer::Integer;
use num_traits::Zero;

/// A probability distribution over values of type `T`.
pub trait Distribution<T> {
    /// samples a value from the distribution.
    fn sample(&self, rng: &mut ThreadRng) -> T;
}

/// Uniformly samples a random non-negative `Int` ...
pub struct IntDistribution {
    /// ... satisfying `_ >= start` for a non-negative `start`
    pub start: Int,
    /// ... and `_ < end`
    pub end: Int,
    /// ... and `_ % divisor == 0` for a positive `divisor`
    pub divisor: Int,
}

impl Distribution<Int> for IntDistribution {
    fn sample(&self, rng: &mut ThreadRng) -> Int {
        let start = self.start.ext();
        let end = self.end.ext();
        let divisor = self.divisor.ext();

        assert!(start >= ExtInt::zero());
        assert!(divisor > ExtInt::zero());

        let start = start.div_ceil(&divisor);
        let end = end.div_ceil(&divisor);

        assert!(start < end);

        let out = rng.gen_bigint_range(&start, &end);
        let out = out * divisor;

        Int::wrap(out)
    }
}


/// A uniform distribution over values of type `T` collected from a finite iterator.
pub struct UniformDistribution<T> {
    omega: Vec<T>,
}

impl<T> UniformDistribution<T> 
    where T: Copy,
{
    /// Returns a uniform distribution based on `iter`.
    pub fn new(iter: impl Iterator<Item = T>) -> Self {
        Self {
            omega: iter.collect(),
        }
    }
}

impl<T> Distribution<T> for UniformDistribution<T> 
    where T: Copy,
{
    fn sample(&self, rng: &mut ThreadRng) -> T {
        let len = self.omega.len();
        assert!(len > 0);

        let index = rng.gen_range(0..len);

        self.omega[index]
    }
}

#[test]
fn test_int_distr() {
    let mut rng = rand::thread_rng();
    for (start, end, divisor) in [(0, 8, 4), (2, 5, 4), (0, 3, 3), (1, 4, 3)] {
        let distr = IntDistribution {
            start: start.into(),
            end: end.into(),
            divisor: divisor.into(),
        };
        for _ in 0..20 {
            let v = distr.sample(&mut rng);
            assert!(v >= distr.start);
            assert!(v < distr.end);
            assert!(v % distr.divisor == 0);
        }
    }
}

#[test]
fn test_uniform() {
    let mut rng = rand::thread_rng();
    let iter = 10..30;
    let distr = UniformDistribution::new(iter);

    for _ in 0..20 {
        let v = distr.sample(&mut rng);
        assert!(10 <= v && v < 30);
    }

    let vec = vec!["hello", "world", "!"];
    let distr = UniformDistribution::new(vec.clone().into_iter());

    for _ in 0..20 {
        let v = distr.sample(&mut rng);
        assert!(vec.contains(&v));
    }
}