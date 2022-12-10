use crate::*;

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
    loop {
        // TODO call gc
        machine.step()?;
    }
}

