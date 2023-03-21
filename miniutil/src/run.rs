use crate::*;

use gen_minirust::prelude::NdResult;
use std::collections::HashSet;
use GcCompat;

pub fn run_program(prog: Program) -> TerminationInfo {
    let res: NdResult<!> = try {
        let mut machine = Machine::<BasicMemory>::new(prog)?;
        mark_and_sweep(&machine);
        loop {
            machine.step()?;
            mark_and_sweep(&machine);
        }
    };

    match res.get() {
        Ok(never) => never,
        Err(t) => t,
    }
}

fn mark_and_sweep<M: Memory>(machine: &Machine<M>) {
    let mut set = HashSet::new();
    machine.points_to(&mut set);
    gen_minirust::libspecr::hidden::mark_and_sweep(set);
}
