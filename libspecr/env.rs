use crate::specr::*;
use crate::specr::gccow::{GcCow, GcCompat};
use im::hashset::HashSet;
use im::hashmap::HashMap;

use std::any::Any;

pub struct Set<T>(GcCow<HashSet<T>>);

impl<T> Clone for Set<T> {
    fn clone(&self) -> Self { Set(self.0) }
}
impl<T> Copy for Set<T> {}
impl<T> GcCompat for HashSet<T> {
    fn points_to(&self, _m: &mut std::collections::HashSet<usize>) {}
    fn as_any(&self) -> &dyn Any { self }
}

impl<T> GcCompat for Set<T> {
    fn points_to(&self, m: &mut std::collections::HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}

pub struct Map<K, V>(GcCow<HashMap<K, V>>);

impl<K, V> Clone for Map<K, V> {
    fn clone(&self) -> Self { Map(self.0) }
}
impl<K, V> Copy for Map<K, V> {}
impl<K, V> GcCompat for HashMap<K, V> {
    fn points_to(&self, _m: &mut std::collections::HashSet<usize>) {}
    fn as_any(&self) -> &dyn Any { self }
}

impl<K, V> GcCompat for Map<K, V> {
    fn points_to(&self, m: &mut std::collections::HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}

#[derive(Copy, Clone, Debug, Hash)]
pub struct String(pub(in crate::specr) GcCow<std::string::String>);

impl GcCompat for String {
    fn points_to(&self, m: &mut std::collections::HashSet<usize>) {
        self.0.points_to(m);
    }
    fn as_any(&self) -> &dyn Any { self }
}

impl GcCompat for std::string::String {
    fn points_to(&self, m: &mut std::collections::HashSet<usize>) {}
    fn as_any(&self) -> &dyn Any { self }
}

pub fn default<T: Default>() -> T {
	T::default()
}

pub fn pick<T>(_f: impl Fn(T) -> bool) -> Nondet<T> { todo!() }
pub fn predict<T>(_f: impl Fn(T) -> bool) -> Nondet<T> { todo!() }

#[derive(Copy, Clone)]
pub enum Endianness {
    LittleEndian,
    BigEndian
}
pub use Endianness::*;
