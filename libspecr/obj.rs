use crate::libspecr::*;

/// A PseudoRust object.
/// Most of the types used in PseudoRust (std lib types and user-defined types)
/// will implement this trait.
///
/// A notable exception is closures.
pub trait Obj: GcCompat + Copy + Debug + Eq + Hash + 'static {}
impl<T> Obj for T where T: GcCompat + Copy + Debug + Eq + Hash + 'static {}
