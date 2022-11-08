use crate::specr::*;

use std::ops::{Try, FromResidual, ControlFlow};

pub struct ArgAbi;
pub type BbName = String;
pub type LocalName = String;
pub type FnName = String;

pub struct Nondet<T>(pub(in crate::specr) T);

impl<T, E> Try for Nondet<Result<T, E>> {
    type Output = T;
    type Residual = Nondet<Result<!, E>>;

    fn from_output(output: Self::Output) -> Self {
        Nondet(Ok(output))
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self.0 {
            Ok(x) => ControlFlow::Continue(x),
            Err(e) => ControlFlow::Break(Nondet(Err(e))),
        }
    }
}

impl<T, E> FromResidual<Nondet<Result<!, E>>> for Nondet<Result<T, E>> {
    fn from_residual(residual: Nondet<Result<!, E>>) -> Self {
        match residual.0 {
            Ok(x) => match x {},
            Err(e) => Nondet(Err(e))
        }
    }
}
