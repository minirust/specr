use std::rc::Rc;
use crate::Nondet;

pub fn deref_rc<T: Clone>(rc: &Rc<T>) -> T {
    (**rc).clone()
}

pub trait MonadicReturn<T> {
	fn monadic_return(t: T) -> Self;
}

pub fn ret<I, O: MonadicReturn<I>>(i: I) -> O {
	O::monadic_return(i)
}

impl<T> MonadicReturn<T> for T { fn monadic_return(t: T) -> Self { t } }
impl<T> MonadicReturn<T> for Option<T> { fn monadic_return(t: T) -> Self { Some(t) } }
impl<T, E> MonadicReturn<T> for Result<T, E> { fn monadic_return(t: T) -> Self { Ok(t) } }
impl<T> MonadicReturn<T> for Nondet<T> { fn monadic_return(t: T) -> Self { Nondet(t) } }
impl<T, E> MonadicReturn<T> for Nondet<Result<T, E>> { fn monadic_return(t: T) -> Self { Nondet(Ok(t)) } }

#[test]
fn monadic_return_test() {
	let _: i32 = ret(5);
	let _: Option<i32> = ret(5);
	let _: Result<i32, ()> = ret(5);
	let _: Nondet<i32> = ret(5);
	let _: Nondet<Result<i32, ()>> = ret(5);
}

// TODO this function panics in some cases. I should handle those cases.
pub fn bigint_to_usize(b: crate::BigInt) -> usize {
    let (sign, digits) = b.0.to_u64_digits();
    if sign == num_bigint::Sign::Minus {
        panic!("cannot convert negative number to usize");
    }
    if digits.len() > 1 {
        panic!("number too large to fit into usize!");
    }

    *digits.get(0).unwrap_or(&0) as usize
}

pub fn vec_to_list<T>(v: Vec<T>) -> crate::list::List<T> {
    crate::list::List(v)
}

