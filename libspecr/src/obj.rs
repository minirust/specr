use std::hash::Hash;
use std::fmt::Debug;

use crate::*;

/// A specr lang object.
/// Most of the types used in specr lang (std lib types and user-defined types)
/// will implement this trait.
///
/// A notable exception is closures.
pub trait Obj: GcCompat + Copy + Debug + Eq + Hash {}
impl<T> Obj for T where T: GcCompat + Copy + Debug + Eq + Hash {}
