#![cfg(test)]

extern crate gen_minirust;
extern crate minisyntax;

pub use minisyntax::run::*;
pub use minisyntax::build::*;
pub use minisyntax::dump::*;

pub use gen_minirust::lang::*;
pub use gen_minirust::mem::*;
pub use gen_minirust::prelude::*;

pub use gen_minirust::specr::*;
pub use gen_minirust::specr::prelude::*;
pub use gen_minirust::specr::hidden::*;

pub use std::format;
pub use std::string::String;
pub use gen_minirust::prelude::NdResult;

mod pass;
mod ub;
mod ill_formed;

pub fn assert_stop(prog: Program) {
    assert_eq!(run_program(prog), Outcome::Stop);
}

pub fn assert_ub(prog: Program, msg: &str) {
    assert_eq!(run_program(prog), Outcome::Ub(msg.to_string()));
}

pub fn assert_ill_formed(prog: Program) {
    assert_eq!(run_program(prog), Outcome::IllFormed);
}
