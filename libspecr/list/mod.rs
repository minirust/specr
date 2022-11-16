use crate::specr::BigInt;
use crate::specr::hidden::{bigint_to_usize, list_from_elem};
use crate::specr::gccow::{GcCow, GcCompat};

use std::collections::HashSet;
use std::any::Any;

use im::vector::Vector;

mod iter;
mod func;

#[derive(Copy, Clone)]
pub struct List<T>(pub(in crate::specr) GcCow<Vector<T>>);

pub macro list {
	() => { List::new() },
	($start:expr $(,$a:expr)*) => { [$start $(,$a)* ].into_iter().collect::<List<_>>() },
	($a:expr ; $b:expr) => { list_from_elem($a, BigInt::from($b)) },
}

impl<T: Clone> GcCompat for Vector<T> where T: GcCompat {
    fn points_to(&self, m: &mut HashSet<usize>) {
        for i in self.iter() {
            i.points_to(m);
        }
    }
    fn as_any(&self) -> &dyn Any { self}
}

impl<T> GcCompat for List<T> where T: GcCompat + Clone {
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self}
}
