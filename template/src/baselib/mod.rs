use std::collections::{HashSet, HashMap};
use crate::mem::BasicMemory;
use crate::prelude::{Endianness, Signedness, Result, NdResult, TerminationInfo};
use std::ops::FromResidual;

pub mod prelude {
    pub use super::{
        BigInt,
        List,
        Set,
        Map,
        pick,
        predict,
        default,
    };
}

mod bigint;
pub use bigint::*;

pub type List<T> = Vec<T>;
pub type Set<T> = HashSet<T>;
pub type Map<K, V> = HashMap<K, V>;
pub type Memory = BasicMemory;

pub fn default<T: Default>() -> T {
	T::default()
}

pub struct Size;
pub struct Align;

pub struct ArgAbi;

impl Endianness {
    fn decode<const N: usize>(self, _signed: Signedness, _bytes: [u8; N]) -> BigInt { todo!() }
    fn encode<const N: usize>(self, _signed: Signedness, _int: BigInt) -> Option<[u8; N]> { todo!() }
}

pub type BbName = String;
pub type LocalName = String;
pub type FnName = String;

pub struct Nondet<T>(pub T);
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

// Yeet
use std::ops::Yeet;

impl<T> FromResidual<Yeet<TerminationInfo>> for NdResult<T> {
	fn from_residual(Yeet(x): Yeet<TerminationInfo>) -> Self {
		Nondet(Err(x))
	}
}
