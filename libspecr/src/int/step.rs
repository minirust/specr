use crate::int::*;

use std::iter::Step;

impl Step for Int {
    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
        match (*end - *start).try_to_usize() {
            Some(diff) => (diff, Some(diff)),
            None => (usize::MAX, None),
        }
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        Some(start + count)
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        Some(start - count)
    }

    fn forward_overflowing(start: Self, count: usize) -> (Self, bool) {
        (start + count, false)
    }

    fn backward_overflowing(start: Self, count: usize) -> (Self, bool) {
        (start - count, false)
    }
}
