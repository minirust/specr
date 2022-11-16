use crate::specr::BigInt;
use crate::specr::hidden::{bigint_to_usize, vec_to_list};
use crate::specr::gccow::{GcCow, GcCompat};

use std::collections::HashSet;
use std::any::Any;

use im::vector::Vector;

mod index;
mod trait_impls;
mod func;

#[derive(Copy, Clone)]
pub struct List<T>(pub(in crate::specr) GcCow<Vector<T>>);

pub macro list {
	() => { List::new() },
	($start:expr $(,$a:expr)*) => { vec_to_list(vec![$start $(,$a)* ]) },
	($a:expr ; $b:expr) => {
        vec_to_list(
            vec![$a;
                bigint_to_usize(BigInt::from($b))
            ]
        )
    },
}

impl<T> GcCompat for Vector<T> {
    fn points_to(&self, _m: &mut HashSet<usize>) {}
    fn as_any(&self) -> &dyn Any { self}
}

impl<T> GcCompat for List<T> {
    fn points_to(&self, m: &mut HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self}
}
