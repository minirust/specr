use crate::*;

use gen_minirust::prelude::NdResult;
use std::collections::HashSet;

// Run the program and return its TerminationInfo.
// We fix `BasicMemory` as a memory for now.
pub fn run_program(prog: Program) -> TerminationInfo {
    let res: NdResult<!> = try {
        let mut machine = Machine::<BasicMemory>::new(prog)?;

        loop {
            machine.step()?;
            mark_and_sweep(&machine);
        }
    };

    // Extract the TerminationInfo from the `NdResult<!>`.
    let res: Result<!, TerminationInfo> = res.get_internal();
    match res {
        Ok(never) => never,
        Err(t) => t,
    }
}

// This drops everything not reachable from the machine.
fn mark_and_sweep<M: Memory>(machine: &Machine<M>) {
    // `set` is the set of gc pointers directly pointed to by `machine`.
    let mut set = HashSet::new();
    machine.points_to(&mut set);

    gen_minirust::libspecr::hidden::mark_and_sweep(set);
}
