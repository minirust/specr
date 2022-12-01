use crate::*;

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
impl<T, E> const MonadicReturn<T> for NdResult<T, E> { fn monadic_return(t: T) -> Self { NdResult(Ok(t)) } }

/// This is hack that makes this `let x: ! = ret(loop {});` work.
impl<E> const MonadicReturn<()> for NdResult<!, E> { fn monadic_return(t: ()) -> Self { panic!() } }

#[test]
fn monadic_return_test() {
	let _: i32 = ret(5);
	let _: Option<i32> = ret(5);
	let _: Result<i32, ()> = ret(5);
	let _: Nondet<i32> = ret(5);
	let _: NdResult<i32, ()> = ret(5);
}


