use crate::int::*;

use std::iter::Step;

impl Step for Int {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        (*end - *start).try_to_usize()
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        Some(start + count)
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        Some(start - count)
    }
}
