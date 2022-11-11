use crate::specr::BigInt;
use crate::specr::hidden::{bigint_to_usize, vec_to_list};

mod index;
mod trait_impls;
mod func;

#[derive(Clone)]
pub struct List<T>(pub(in crate::specr) Vec<T>);

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
