use crate::libspecr::*;

use std::ops::*;
use std::cmp::Ordering;

use crate::prelude::Align;


impl PartialOrd for Align {
    fn partial_cmp(&self, rhs: &Align) -> Option<Ordering> {
        self.bytes().partial_cmp(&rhs.bytes())
    }
}

impl Ord for Align {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bytes().cmp(&other.bytes())
    }
}
