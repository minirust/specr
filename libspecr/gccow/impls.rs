use super::*;
use crate::specr::gccow::GcCompat;
use std::collections::HashSet;
use std::any::Any;

// TODO add other impls
impl GcCompat for () {
    fn points_to(&self, _m: &mut HashSet<usize>) { }
    fn as_any(&self) -> &dyn Any { self }
}
