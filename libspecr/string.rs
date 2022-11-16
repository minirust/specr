use crate::specr::*;
use crate::specr::gccow::{GcCow, GcCompat};
use im::hashset::HashSet;
use im::hashmap::HashMap;

use std::any::Any;

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub struct String(pub(in crate::specr) GcCow<std::string::String>);

impl GcCompat for String {
    fn points_to(&self, m: &mut std::collections::HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl GcCompat for std::string::String {
    fn points_to(&self, m: &mut std::collections::HashSet<usize>) {}
    fn as_any(&self) -> &dyn Any { self }
}
