// TODO this file shall be part of Minirust at some point.
// It doesn't belong to minisyntax.

use crate::*;

use std::collections::HashSet;
use GcCompat;

#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    IllFormed, // program not well-formed
    Stop, // program stopped normally
    Ub(String), // program raised UB
}

pub fn run_program(prog: Program) -> Outcome {
    fn run_impl<M: Memory>(mut machine: Machine<M>) -> NdResult<!> {
        loop {
            machine.step()?;
            mark_and_sweep(&machine);
        }
    }

    let Some(machine) = Machine::<BasicMemory>::new(prog) else {
        return Outcome::IllFormed;
    };
    mark_and_sweep(&machine);

    let x = run_impl(machine).get();
    let t_info = match x {
        Ok(never) => never,
        Err(t_info) => t_info,
    };

    match t_info {
        TerminationInfo::Ub(err) => Outcome::Ub(err.get()),
        TerminationInfo::MachineStop => Outcome::Stop,
        _ => todo!(),
    }
}

fn mark_and_sweep<M: Memory>(machine: &Machine<M>) {
    let mut set = HashSet::new();
    machine.points_to(&mut set);
    gen_minirust::libspecr::hidden::mark_and_sweep(set);
}
