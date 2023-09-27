use crate::*;

/// Satisfied by types that can be constructed from some inner types.
pub trait MonadicReturn {
    /// The inner type.
    type Inner;
    /// Wraps a value of `Self::Inner` into a `Self`.
    fn monadic_return(t: Self::Inner) -> Self;
}

/// Wraps a value `i` as `Some(i)`, `Ok(i)` or something similar of type `T`.
pub fn ret<T: MonadicReturn>(i: T::Inner) -> T {
    T::monadic_return(i)
}

impl<T> MonadicReturn for Option<T> {
    type Inner = T;
    fn monadic_return(t: T) -> Self { Some(t) }
}

impl<T, E> MonadicReturn for Result<T, E> {
    type Inner = T;
    fn monadic_return(t: T) -> Self { Ok(t) }
}

impl<T> MonadicReturn for Nondet<T> {
    type Inner = T;
    fn monadic_return(t: T) -> Self { Nondet(t) }
}

impl<T, E> MonadicReturn for NdResult<T, E> {
    type Inner = T;
    fn monadic_return(t: T) -> Self { NdResult(Ok(t)) }
}

#[test]
fn monadic_return_test() {
    let _: Option<i32> = ret(5);
    let _: Result<i32, ()> = ret(5);
    let _: Nondet<i32> = ret(5);
    let _: NdResult<i32, ()> = ret(5);
}
