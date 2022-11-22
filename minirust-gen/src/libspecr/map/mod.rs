use crate::libspecr::*;

mod func;
mod impls;
mod iter;

pub struct Map<K, V>(pub(in crate::libspecr) GcCow<IMHashMap<K, V>>);
