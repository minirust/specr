use crate::libspecr::*;

mod func;
mod impls;

pub struct Set<T>(pub(in crate::libspecr) GcCow<IMHashSet<T>>);
