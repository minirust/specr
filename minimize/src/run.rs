use crate::*;

use std::collections::HashSet;
use specr::GcCompat;

pub fn run_program(prog: mini::Program) {
    let specr::NdResult(Err(t_info)) = run_program_impl(prog) else { unreachable!() };

    match t_info {
        mini::TerminationInfo::MachineStop => { /* silent exit. */ },
        mini::TerminationInfo::Ub(err) => println!("UB: {}", err),
        _ => todo!(),
    }
}


fn run_program_impl(prog: mini::Program) -> mini::NdResult<!> {
    let mut machine = mini::Machine::<mini::BasicMemory>::new(prog).unwrap();
    mark_and_sweep(&machine);

    loop {
        machine.step()?;
        mark_and_sweep(&machine);
    }
}

fn mark_and_sweep<M: mini::Memory>(machine: &mini::Machine<M>) {
    let mut set = HashSet::new();
    machine.points_to(&mut set);
    specr::mark_and_sweep(set);
}
