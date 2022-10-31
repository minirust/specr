use std::collections::{HashSet, HashMap};
use crate::mem::basic::BasicMemory;
use crate::lang::prelude::Endianess;
use crate::prelude::{Signedness, Result, NdResult, TerminationInfo};

pub type List<T> = Vec<T>;
pub type Set<T> = HashSet<T>;
pub type Map<K, V> = HashMap<K, V>;
pub type Memory = BasicMemory;

pub fn default<T: Default>() -> T {
	T::default()
}

pub struct BigInt;
pub struct Size;
pub struct Align;

pub struct ArgAbi;

pub struct MyEndianess;
impl Endianess for MyEndianess {
    fn decode<const N: usize>(self, _signed: Signedness, _bytes: [u8; N]) -> BigInt { todo!() }
    fn encode<const N: usize>(self, _signed: Signedness, _int: BigInt) -> Option<[u8; N]> { todo!() }

}
pub const ENDIANESS: MyEndianess = MyEndianess;

pub type BbName = String;
pub type LocalName = String;
pub type FnName = String;

pub const PTR_SIZE: usize = 32;

pub struct Nondet<T>(T);
pub fn pick<T>(f: impl Fn(T) -> bool) -> Nondet<T> { todo!() }
pub fn predict<T>(f: impl Fn(T) -> bool) -> Nondet<T> { todo!() }

pub trait PartialOrd2 {
	fn le(self, other: Self) -> bool;
}

#[macro_export]
macro_rules! list {
	() => { vec![] };
	($start:expr $(,$a:expr)*) => { vec![$start $(,$a)* ] };
	($a:expr ; $b:expr) => { vec![$a ; $b] };
}

// OPERATORS
use std::ops::*;

impl Neg for BigInt {
	type Output = Self;
	fn neg(self) -> Self { todo!() }
}

impl Add for BigInt {
	type Output = Self;
	fn add(self, other: Self) -> Self { todo!() }
}

impl Sub for BigInt {
	type Output = Self;
	fn sub(self, other: Self) -> Self { todo!() }
}

impl Mul for BigInt {
	type Output = Self;
	fn mul(self, other: Self) -> Self { todo!() }
}

impl PartialEq<usize> for BigInt {
	fn eq(&self, other: &usize) -> bool { todo!() }
}

impl BigInt {
	pub fn checked_div(self, other: BigInt) -> Option<BigInt> {
		todo!()
	}

	pub fn modulo(self, _: Signedness, other: usize) -> BigInt {
		todo!()
	}
}

// MonadicReturn

pub trait MonadicReturn<T> {
	fn monadic_return(t: T) -> Self;
}

pub fn ret<I, O: MonadicReturn<I>>(i: I) -> O {
	O::monadic_return(i)
}

impl<T> MonadicReturn<T> for T { fn monadic_return(t: T) -> Self { t } }
impl<T> MonadicReturn<T> for Option<T> { fn monadic_return(t: T) -> Self { Some(t) } }
impl<T> MonadicReturn<T> for Result<T> { fn monadic_return(t: T) -> Self { Ok(t) } }
impl<T> MonadicReturn<T> for Nondet<T> { fn monadic_return(t: T) -> Self { Nondet(t) } }
impl<T> MonadicReturn<T> for NdResult<T> { fn monadic_return(t: T) -> Self { Nondet(Ok(t)) } }

#[test]
fn monadic_return_test() {
	let _: i32 = ret(5);
	let _: Option<i32> = ret(5);
	let _: Result<i32> = ret(5);
	let _: Nondet<i32> = ret(5);
	let _: NdResult<i32> = ret(5);
}

// Yeet
use std::ops::Yeet;

impl<T> FromResidual<Yeet<TerminationInfo>> for NdResult<T> {
	fn from_residual(Yeet(x): Yeet<TerminationInfo>) -> Self {
		Nondet(Err(x))
	}
}
