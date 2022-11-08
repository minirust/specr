use crate::baselib::*;

pub struct ArgAbi;
pub type BbName = String;
pub type LocalName = String;
pub type FnName = String;

pub struct Nondet<T>(pub(in crate::baselib) T);
