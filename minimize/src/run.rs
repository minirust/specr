use crate::*;

use std::collections::HashSet;
use specr::GcCompat;

#[derive(Debug, PartialEq, Eq)]
pub enum Outcome {
    Unwell, // program not well-formed
    Stop, // program stopped normally
    Ub(String), // program raised UB
}

pub fn run_program(prog: mini::Program) -> Outcome {
    fn run_impl<M: mini::Memory>(mut machine: mini::Machine<M>) -> mini::NdResult<!> {
        loop {
            machine.step()?;
            mark_and_sweep(&machine);
        }
    }

    let Some(machine) = mini::Machine::<mini::BasicMemory>::new(prog) else {
        return Outcome::Unwell;
    };
    mark_and_sweep(&machine);

    let specr::NdResult(x) = run_impl(machine);
    let t_info = match x {
        Ok(never) => never,
        Err(t_info) => t_info,
    };

    match t_info {
        mini::TerminationInfo::Ub(err) => Outcome::Ub(err.0.get()),
        mini::TerminationInfo::MachineStop => Outcome::Stop,
        _ => todo!(),
    }
}

fn mark_and_sweep<M: mini::Memory>(machine: &mini::Machine<M>) {
    let mut set = HashSet::new();
    machine.points_to(&mut set);
    specr::mark_and_sweep(set);
}
