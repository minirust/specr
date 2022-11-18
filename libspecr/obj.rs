use crate::libspecr::*;

pub trait Obj: GcCompat + Copy + Debug {}
impl<T: GcCompat + Copy + Debug> Obj for T {}
