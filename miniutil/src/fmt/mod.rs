use crate::*;

mod expr;
use expr::*;

mod ty;
use ty::*;

pub fn dump_program(prog: Program) {
    let s = fmt_program(prog);
    println!("{s}");
}

pub fn fmt_program(prog: Program) -> String {
    let mut comptypes: Vec<CompType> = Vec::new();
    let functions_string = fmt_functions(prog, &mut comptypes);
    let globals_string = fmt_globals(prog.globals);
    let comptypes_string = fmt_comptypes(comptypes);

    comptypes_string + &functions_string + &globals_string
}

fn fmt_functions(prog: Program, comptypes: &mut Vec<CompType>) -> String {
    let mut out = String::new();
    let mut fns: Vec<(FnName, Function)> = prog.functions.iter().collect();

    // functions are formatted in the order given by their name.
    fns.sort_by_key(|(FnName(name), _fn)| *name);

    for (fn_name, f) in fns {
        let start = prog.start == fn_name;
        out += &fmt_function(fn_name, f, start, comptypes);
    }

    out
}

fn fmt_comptypes(mut comptypes: Vec<CompType>) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < comptypes.len() {
        let c = comptypes[i];
        let comptype_index = CompTypeIndex { idx: i };

        // A call to `fmt_comptype` might push further comptypes.
        // Hence, we cannot use an iterator here.
        let s = &*fmt_comptype(comptype_index, c, &mut comptypes);

        out += s;

        i += 1;
    }

    out
}

fn fmt_bytes(bytes: List<Option<u8>>) -> String {
    let b: Vec<_> = bytes
        .iter()
        .map(|x| match x {
            Some(u) => format!("{:02x?}", u),
            None => String::from("__"),
        })
        .collect();

    b.join(" ")
}

fn fmt_globals(globals: Map<GlobalName, Global>) -> String {
    let mut out = String::new();

    let mut globals: Vec<(GlobalName, Global)> = globals.iter().collect();

    globals.sort_by_key(|(GlobalName(name), _global)| *name);

    for (gname, global) in globals {
        out += &fmt_global(gname, global);
    }
    out
}

fn fmt_global(gname: GlobalName, global: Global) -> String {
    let gname_str = fmt_global_name(gname);
    let bytes_str = fmt_bytes(global.bytes);
    let align = global.align.bytes();
    let mut out = format!(
        "{gname_str} {{
  bytes = [{bytes_str}],
  align = {align} bytes,\n"
    );
    for (i, rel) in global.relocations {
        let i = i.bytes();
        let rel_str = fmt_relocation(rel);
        out += &format!("  at byte {i}: {rel_str},\n");
    }
    out += "}\n\n";
    out
}

fn fmt_relocation(relocation: Relocation) -> String {
    let gname = fmt_global_name(relocation.name);

    if relocation.offset.bytes() == 0 {
        gname
    } else {
        let offset = relocation.offset.bytes();
        format!("({gname} + {offset})")
    }
}

fn fmt_function(
    fn_name: FnName,
    f: Function,
    start: bool,
    comptypes: &mut Vec<CompType>,
) -> String {
    let start_str = if start { "start " } else { "" };
    let fn_name = fmt_fn_name(fn_name);
    let args: Vec<_> = f
        .args
        .iter()
        .map(|(x, _)| {
            let ident = fmt_local_name(x);
            let ty = fmt_ptype(f.locals.index_at(x), comptypes);

            format!("{ident}: {ty}")
        })
        .collect();
    let args = args.join(", ");

    let mut ret_ty = String::from("none");
    if let Some((ret, _)) = f.ret {
        ret_ty = fmt_ptype(f.locals.index_at(ret), comptypes);
    }
    let mut out = format!("{start_str}fn {fn_name}({args}) -> {ret_ty} {{\n");

    // Format locals
    let mut locals: Vec<_> = f.locals.keys().collect();
    locals.sort_by_key(|l| l.0.get_internal());
    for l in locals {
        let ty = f.locals.index_at(l);
        let local = fmt_local_name(l);
        let ptype = fmt_ptype(ty, comptypes);
        out += &format!("  let {local}: {ptype};\n");
    }

    // Blocks are formatted in order.
    let mut blocks: Vec<(BbName, BasicBlock)> = f.blocks.iter().collect();
    blocks.sort_by_key(|(BbName(name), _block)| *name);
    for (bb_name, bb) in blocks {
        let start = f.start == bb_name;
        out += &fmt_bb(bb_name, bb, start, comptypes);
    }
    out += "}\n\n";

    out
}

fn fmt_bb(bb_name: BbName, bb: BasicBlock, start: bool, comptypes: &mut Vec<CompType>) -> String {
    let name = bb_name.0.get_internal();
    let start_str = match start {
        true => "start ",
        false => "",
    };
    let mut out = format!("  {start_str}bb{name}:\n");

    for st in bb.statements.iter() {
        out += &fmt_statement(st, comptypes);
        out.push('\n');
    }
    out += &fmt_terminator(bb.terminator, comptypes);
    out.push('\n');
    out
}

fn fmt_statement(st: Statement, comptypes: &mut Vec<CompType>) -> String {
    match st {
        Statement::Assign {
            destination,
            source,
        } => {
            let left = fmt_place_expr(destination, comptypes);
            let right = fmt_value_expr(source, comptypes);
            format!("    {left} = {right};")
        }
        Statement::Finalize { place, fn_entry } => {
            let place = fmt_place_expr(place, comptypes);
            format!("    Finalize({place}, {fn_entry});")
        }
        Statement::StorageLive(local) => {
            let local = fmt_local_name(local);
            format!("    StorageLive({local});")
        }
        Statement::StorageDead(local) => {
            let local = fmt_local_name(local);
            format!("    StorageDead({local});")
        }
    }
}

fn fmt_call(
    callee: &str,
    arguments: List<ValueExpr>,
    ret: Option<PlaceExpr>,
    next_block: Option<BbName>,
    comptypes: &mut Vec<CompType>,
) -> String {
    let args: Vec<_> = arguments
        .iter()
        .map(|x| fmt_value_expr(x, comptypes))
        .collect();
    let args = args.join(", ");

    let mut r = String::from("none");
    if let Some(ret) = ret {
        r = fmt_place_expr(ret, comptypes);
    }
    let mut next = String::new();
    if let Some(next_block) = next_block {
        let next_str = fmt_bb_name(next_block);
        next = format!(" -> {next_str}");
    }

    format!("    {r} = {callee}({args}){next};")
}

fn fmt_terminator(t: Terminator, comptypes: &mut Vec<CompType>) -> String {
    match t {
        Terminator::Goto(bb) => {
            let bb = fmt_bb_name(bb);
            format!("    goto -> {bb};")
        }
        Terminator::If {
            condition,
            then_block,
            else_block,
        } => {
            let branch_expr = fmt_value_expr(condition, comptypes);
            let then_bb = fmt_bb_name(then_block);
            let else_bb = fmt_bb_name(else_block);
            format!(
                "    if {branch_expr} {{
      goto -> {then_bb};
    }} else {{
      goto -> {else_bb};
    }}"
            )
        }
        Terminator::Unreachable => {
            format!("    unreachable;")
        }
        Terminator::Call {
            callee,
            arguments,
            ret,
            next_block,
        } => {
            let callee = fmt_value_expr(callee, comptypes);
            let arguments = arguments.iter().map(|(x, _)| x).collect();
            let ret = ret.map(|(x, _)| x);
            fmt_call(&callee, arguments, ret, next_block, comptypes)
        }
        Terminator::Return => {
            format!("    return;")
        }
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
                Intrinsic::Allocate => "allocate",
                Intrinsic::Deallocate => "deallocate",
            };
            fmt_call(callee, arguments, ret, next_block, comptypes)
        }
    }
}

fn fmt_bb_name(bb: BbName) -> String {
    let id = bb.0.get_internal();
    format!("bb{id}")
}

fn fmt_fn_name(fn_name: FnName) -> String {
    let id = fn_name.0.get_internal();
    format!("f{id}")
}

fn fmt_comptype_index(comptype_index: CompTypeIndex) -> String {
    let id = comptype_index.idx;
    format!("T{id}")
}
