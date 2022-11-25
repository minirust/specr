use crate::libspecr::*;

mod func;
mod impls;
mod iter;

pub struct Set<T: Obj>(pub(in crate::libspecr) GcCow<IMHashSet<T>>);
