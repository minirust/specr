use std::ops::*;

use crate::specr::BigInt;
use crate::specr::hidden::bigint_to_usize;

use crate::specr::list::List;

// BigInt:
impl<T> Index<BigInt> for List<T> {
    type Output = T;

    fn index(&self, other: BigInt) -> &T {
        let other = bigint_to_usize(other);
        &self.0[other]
    }
}

impl<T> IndexMut<BigInt> for List<T> {
    fn index_mut(&mut self, other: BigInt) -> &mut T {
        let other = bigint_to_usize(other);
        &mut self.0[other]
    }
}

// Range<BigInt>:
impl<T> Index<Range<BigInt>> for List<T> {
    type Output = [T];

    fn index(&self, range: Range<BigInt>) -> &[T] {
        let start = bigint_to_usize(range.start);
        let end = bigint_to_usize(range.end);
        &self.0[start..end]
    }
}

impl<T> IndexMut<Range<BigInt>> for List<T> {
    fn index_mut(&mut self, range: Range<BigInt>) -> &mut [T] {
        let start = bigint_to_usize(range.start);
        let end = bigint_to_usize(range.end);
        &mut self.0[start..end]
    }
}

// RangeFrom<BigInt>:
impl<T> Index<RangeFrom<BigInt>> for List<T> {
    type Output = [T];

    fn index(&self, range: RangeFrom<BigInt>) -> &[T] {
        let start = bigint_to_usize(range.start);
        &self.0[start..]
    }
}

impl<T> IndexMut<RangeFrom<BigInt>> for List<T> {
    fn index_mut(&mut self, range: RangeFrom<BigInt>) -> &mut [T] {
        let start = bigint_to_usize(range.start);
        &mut self.0[start..]
    }
}

// RangeTo<BigInt>:
impl<T> Index<RangeTo<BigInt>> for List<T> {
    type Output = [T];

    fn index(&self, range: RangeTo<BigInt>) -> &[T] {
        let end = bigint_to_usize(range.end);
        &self.0[..end]
    }
}

impl<T> IndexMut<RangeTo<BigInt>> for List<T> {
    fn index_mut(&mut self, range: RangeTo<BigInt>) -> &mut [T] {
        let end = bigint_to_usize(range.end);
        &mut self.0[..end]
    }
}
