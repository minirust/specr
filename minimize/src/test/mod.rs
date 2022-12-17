use crate::*;

mod build;
use build::*;

mod pass;
mod fail;

pub fn assert_ub(prog: Program, msg: &str) {
    assert_eq!(run_program(prog), Outcome::Ub(msg.to_string()));
}

pub fn assert_stop(prog: Program) {
    assert_eq!(run_program(prog), Outcome::Stop);
}

pub fn assert_unwell(prog: Program) {
    assert_eq!(run_program(prog), Outcome::Unwell);
}
