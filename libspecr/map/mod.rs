use crate::libspecr::*;

mod func;
mod impls;

pub struct Map<K, V>(pub(in crate::libspecr) GcCow<IMHashMap<K, V>>);
