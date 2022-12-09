use crate::*;

pub fn run_program(prog: mini::Program) {
    let mut machine = mini::Machine::<mini::BasicMemory>::new(prog).unwrap();
    loop {
        // TODO call gc
        machine.step();
    }
}
