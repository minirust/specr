use crate::*;

use rand::rngs::ThreadRng;
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
        let start = self.start.into_inner();
        let end = self.end.into_inner();
        let divisor = self.divisor.into_inner();

        assert!(start >= BigInt::zero());
        assert!(divisor > BigInt::zero());

        let start = start.div_ceil(&divisor);
        let end = end.div_ceil(&divisor);

        assert!(start < end);

        let out = rng.gen_bigint_range(&start, &end);
        let out = out * divisor;

        Int::wrap(out)
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
