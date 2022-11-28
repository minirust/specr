use crate::*;

mod ret;
pub use ret::*;

mod obj;
pub use obj::*;

// The GcCow primites are only accessible through the `hidden` module.
pub use crate::gccow::{GcCow, GcCompat};

// TODO this function panics in some cases. I should handle those cases.
pub fn int_to_usize(b: Int) -> usize {
    let (sign, digits) = b.ext().to_u64_digits();
    if sign == num_bigint::Sign::Minus {
        panic!("cannot convert negative number to usize");
    }
    if digits.len() > 1 {
        panic!("number too large to fit into usize!");
    }

    *digits.get(0).unwrap_or(&0) as usize
}

pub fn list_from_elem<T: Obj>(elem: T, n: Int) -> List<T> {
    let n = int_to_usize(n);
    let v: im::vector::Vector<T> = std::iter::repeat(elem).take(n).collect();

    List(GcCow::new(v))
}

#[const_trait]
pub trait MonadicReturn<T> {
	fn monadic_return(t: T) -> Self;
}

pub const fn ret<I, O: ~const MonadicReturn<I>>(i: I) -> O {
	O::monadic_return(i)
}

impl<T> const MonadicReturn<T> for T { fn monadic_return(t: T) -> Self { t } }
impl<T> const MonadicReturn<T> for Option<T> { fn monadic_return(t: T) -> Self { Some(t) } }
impl<T, E> const MonadicReturn<T> for Result<T, E> { fn monadic_return(t: T) -> Self { Ok(t) } }
impl<T> const MonadicReturn<T> for Nondet<T> { fn monadic_return(t: T) -> Self { Nondet(t) } }
impl<T, E> const MonadicReturn<T> for Nondet<Result<T, E>> { fn monadic_return(t: T) -> Self { Nondet(Ok(t)) } }

#[test]
fn monadic_return_test() {
	let _: i32 = ret(5);
	let _: Option<i32> = ret(5);
	let _: Result<i32, ()> = ret(5);
	let _: Nondet<i32> = ret(5);
	let _: Nondet<Result<i32, ()>> = ret(5);
}


