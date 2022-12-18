use crate::{Program, FnName, Function, Type, PlaceExpr, ValueExpr, Constant, PtrType, Mutability, BinOp, BinOpInt, Statement, Terminator, LocalName, BbName, IntType, BasicBlock, Signed, Unsigned, Intrinsic, List};
use crate::*;

mod expr;
use expr::*;

pub fn dump_program(prog: &Program) {
    let mut fns: Vec<(_, _)> = prog.functions.iter().collect();
    fns.sort_by_key(|(k, _)| k.0);
    for (fn_name, f) in fns {
        let start = prog.start == fn_name;
        dump_function(fn_name, f, start);
    }
}

fn dump_function(fn_name: FnName, f: Function, start: bool) {
    let start_str = if start {
        "[start] "
    } else { "" };
    let fn_name = fn_name_to_string(fn_name);
    let args: Vec<_> = f.args.iter().map(|(x, _)| {
            let ident = local_name_to_string(x);
            let ty = type_to_string(f.locals.index_at(x).ty);

            format!("{ident}: {ty}")
        }).collect();
    let args = args.join(", ");

    let mut ret_ty = String::from("!!!");
    if let Some((ret, _)) = f.ret {
        ret_ty = type_to_string(f.locals.index_at(ret).ty);
    }
    println!("{start_str}fn {fn_name}({args}) -> {ret_ty} {{");

    // dump locals
    let mut locals: Vec<_> = f.locals.keys().collect();
    locals.sort_by_key(|l| l.0.get());
    for l in locals {
        let ty = f.locals.index_at(l).ty;
        println!("  let {}: {};", local_name_to_string(l), type_to_string(ty));
    }

    let mut blocks: Vec<(_, _)> = f.blocks.iter().collect();
    blocks.sort_by_key(|(k, _)| k.0);
    for (bb_name, bb) in blocks {
        let start = f.start == bb_name;
        dump_bb(bb_name, bb, start);
    }
    println!("}}");
    println!("");
}

fn dump_bb(bb_name: BbName, bb: BasicBlock, start: bool) {
    if start {
        println!("  bb{} [start]:", bb_name.0.get());
    } else {
        println!("  bb{}:", bb_name.0.get());
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
            println!("    StorageLive({});", local_name_to_string(local));
        },
        Statement::StorageDead(local) => {
            println!("    StorageDead({});", local_name_to_string(local));
        },
    }
}

fn dump_call(callee: &str, arguments: List<ValueExpr>, ret: Option<PlaceExpr>, next_block: Option<BbName>) {
    let args: Vec<_> = arguments.iter().map(value_expr_to_string).collect();
    let args = args.join(", ");

    let mut r = String::from("!!!");
    if let Some(ret) = ret {
        r = place_expr_to_string(ret);
    }
    let mut next = String::new();
    if let Some(next_block) = next_block {
        next = format!(" -> {}", bb_name_to_string(next_block));
    }
    println!("    {r} = {callee}({args}){next};");
}

fn dump_terminator(t: Terminator) {
    match t {
        Terminator::Goto(bb) => {
            println!("    goto -> {};", bb_name_to_string(bb));
        },
        Terminator::If {
            condition,
            then_block,
            else_block,
        } => {
            println!("    if {} {{", value_expr_to_string(condition));
            println!("      goto -> {};", bb_name_to_string(then_block));
            println!("    }} else {{");
            println!("      goto -> {};", bb_name_to_string(else_block));
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
            let callee = fn_name_to_string(callee);
            let arguments = arguments.iter().map(|(x, _)| x).collect();
            let ret = ret.map(|(x, _)| x);
            dump_call(&callee, arguments, ret, next_block);
        },
        Terminator::Return => {
            println!("    return;");
        },
        Terminator::CallIntrinsic {
            intrinsic,
            arguments,
            ret,
            next_block,
        } => {
            let callee = match intrinsic {
                Intrinsic::Exit => "exit",
                Intrinsic::PrintStdout => "print",
                Intrinsic::PrintStderr => "eprint",
            };
            dump_call(callee, arguments, ret, next_block);
        },
    }
}
