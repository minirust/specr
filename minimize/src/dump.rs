use crate::*;

// TODO make pretty
pub fn dump_program(prog: &mini::Program) {
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
                dump_statement(st);
            }
            dump_terminator(bb.terminator);
        }
    }
}

fn dump_statement(st: mini::Statement) {
    match st {
        mini::Statement::Assign { destination, source } => {
            println!("    {} = {};", place_expr_to_string(destination), value_expr_to_string(source));
        },
        mini::Statement::Finalize { place, fn_entry } => {
            println!("    Finalize({}, {});", place_expr_to_string(place), fn_entry);
        },
        mini::Statement::StorageLive(local) => {
            println!("    StorageLive({});", local_to_string(local));
        },
        mini::Statement::StorageDead(local) => {
            println!("    StorageDead({});", local_to_string(local));
        },
    }
}

fn dump_terminator(t: mini::Terminator) {
    println!("    {:?};", t);
}

fn place_expr_to_string(p: mini::PlaceExpr) -> String {
    match p {
        mini::PlaceExpr::Local(l) => local_to_string(l),
        mini::PlaceExpr::Deref { .. } => format!("{:?}", p),
        mini::PlaceExpr::Field { root, field } => {
            let root = root.get();
            format!("{}.{}", place_expr_to_string(root), field)
        },
        mini::PlaceExpr::Index { root, index } => {
            let root = root.get();
            let index = index.get();
            format!("{}[{}]", place_expr_to_string(root), value_expr_to_string(index))
        },
    }
}

fn local_to_string(l: mini::LocalName) -> String {
    format!("_{}", l.0.0)
}

fn value_expr_to_string(v: mini::ValueExpr) -> String {
    format!("{:?}", v)
}
