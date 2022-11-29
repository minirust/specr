use gen_minirust::lang::Program;

// TODO make pretty
pub fn dump_program(prog: &Program) {
    for (fname, f) in prog.functions.iter() {
        if prog.start == fname {
            println!("fn f{} [start]:", fname.0.0);
        } else {
            println!("fn f{}:", fname.0.0);
        }

        for (bbname, bb) in f.blocks.iter() {
            if f.start == bbname {
                println!("  bb b{} [start]:", bbname.0.0);
            } else {
                println!("  bb b{}:", bbname.0.0);
            }

            for st in bb.statements.iter() {
                println!("    {:?}", st);
            }
            println!("    {:?}", bb.terminator);
        }
    }
}
