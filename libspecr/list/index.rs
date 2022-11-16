use std::ops::*;

use crate::specr::BigInt;
use crate::specr::hidden::bigint_to_usize;

use crate::specr::list::List;

impl<T> Index<BigInt> for List<T> {
    type Output = T;

    fn index(&self, other: BigInt) -> &T {
        let other = bigint_to_usize(other);
        &self.0.call_ref(|v| &v[other])
    }
}

impl<T> IndexMut<BigInt> for List<T> {
    fn index_mut(&mut self, other: BigInt) -> &mut T {
        let other = bigint_to_usize(other);
        &self.0.call_mut(|v| &mut v[other])
    }
}
