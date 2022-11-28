use crate::libspecr::*;

mod func;
mod impls;
mod iter;

pub struct Set<T>(pub(in crate::libspecr) GcCow<IMHashSet<T>>);
