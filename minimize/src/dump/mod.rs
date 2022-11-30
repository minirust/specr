use crate::mini::*;

mod expr;
use expr::*;

pub fn dump_program(prog: &Program) {
    for (fname, f) in prog.functions.iter() {
        let start = prog.start == fname;
        dump_function(fname, f, start);
    }
}

fn dump_function(fname: FnName, f: Function, start: bool) {
    if start {
        println!("fn f{} [start]:", fname.0.0);
    } else {
        println!("fn f{}:", fname.0.0);
    }

    // dump locals
    for (l, pt) in f.locals {
        println!("  let {}: {};", localname_to_string(l), type_to_string(pt.ty));
    }

    for (bbname, bb) in f.blocks.iter() {
        let start = f.start == bbname;
        dump_bb(bbname, bb, start);
    }
}

fn dump_bb(bbname: BbName, bb: BasicBlock, start: bool) {
    if start {
        println!("  bb{} [start]:", bbname.0.0);
    } else {
        println!("  bb{}:", bbname.0.0);
    }

    for st in bb.statements.iter() {
        dump_statement(st);
    }
    dump_terminator(bb.terminator);
}

fn dump_statement(st: Statement) {
    match st {
        Statement::Assign { destination, source } => {
            println!("    {} = {};", place_expr_to_string(destination), value_expr_to_string(source));
        },
        Statement::Finalize { place, fn_entry } => {
            println!("    Finalize({}, {});", place_expr_to_string(place), fn_entry);
        },
        Statement::StorageLive(local) => {
            println!("    StorageLive({});", localname_to_string(local));
        },
        Statement::StorageDead(local) => {
            println!("    StorageDead({});", localname_to_string(local));
        },
    }
}

fn dump_terminator(t: Terminator) {
    match t {
        Terminator::Goto(bb) => {
            println!("    goto {};", bbname_to_string(bb));
        },
        Terminator::If {
            condition,
            then_block,
            else_block,
        } => {
            println!("    if {} {{", value_expr_to_string(condition));
            println!("      goto {};", bbname_to_string(then_block));
            println!("    }} else {{");
            println!("      Goto {};", bbname_to_string(else_block));
            println!("    }}");
        },
        Terminator::Unreachable => {
            println!("    unreachable;");
        }
        Terminator::Call {
            callee,
            arguments,
            ret,
            next_block,
        } => {
            let args: Vec<_> = arguments.iter().map(|(x, _)| value_expr_to_string(x)).collect();
            let args = args.join(", ");

            let ret = place_expr_to_string(ret.0);
            let callee = fnname_to_string(callee);
            let next = bbname_to_string(next_block);
            println!("    {ret} = {callee}({args}) -> {next};");
        },
        Terminator::Return => {
            println!("    return;");
        }
    }
}
