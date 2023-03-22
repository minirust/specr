use crate::*;

mod expr;
use expr::*;

mod ty;
use ty::*;

// A list of "composite" type, namely a union or a tuple (enums aren't yet supported).
// They are not rendered like normal types, but rather the i'th comptype will be rendered as `Ti`.
// Composite types are formatted before the functions.
type CompTypes = Vec<Type>;

pub fn dump_program(prog: Program) {
    let s = program_to_string(prog);
    println!("{s}");
}

pub fn program_to_string(prog: Program) -> String {
    let mut comptypes: CompTypes = CompTypes::new();
    let functions_string = functions_to_string(prog, &mut comptypes);
    let globals_string = globals_to_string(prog.globals);
    let comptypes_string = comptypes_to_string(comptypes);

    comptypes_string + &functions_string + &globals_string
}

fn functions_to_string(prog: Program, comptypes: &mut CompTypes) -> String {
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

fn comptypes_to_string(mut comptypes: CompTypes) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < comptypes.len() {
        let c = comptypes[i];

        // A call to `fmt_comptype` might push further comptypes.
        // Hence, we cannot use an iterator here.
        let s = &*fmt_comptype(i, c, &mut comptypes);

        out += s;

        i += 1;
    }

    out
}

fn bytes_to_string(bytes: List<Option<u8>>) -> String {
    let b: Vec<_> = bytes
        .iter()
        .map(|x| match x {
            Some(u) => format!("{:02x?}", u),
            None => String::from("__"),
        })
        .collect();

    b.join(" ")
}

fn globals_to_string(globals: Map<GlobalName, Global>) -> String {
    let mut out = String::new();

    let mut globals: Vec<(GlobalName, Global)> = globals.iter().collect();

    globals.sort_by_key(|(GlobalName(name), _global)| *name);

    for (gname, global) in globals {
        out += &fmt_global(gname, global);
    }
    out
}

fn fmt_global(gname: GlobalName, global: Global) -> String {
    let gname_str = global_name_to_string(gname);
    let bytes_str = bytes_to_string(global.bytes);
    let align = global.align.bytes();
    let mut out = format!(
"{gname_str} {{
  bytes = [{bytes_str}],
  align = {align} bytes,\n");
    for (i, rel) in global.relocations {
        let i = i.bytes();
        let rel_str = relocation_to_string(rel);
        out += &format!("  at byte {i}: {rel_str},\n");
    }
    out += "}\n\n";
    out
}

pub fn relocation_to_string(relocation: Relocation) -> String {
    let gname = global_name_to_string(relocation.name);

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
    comptypes: &mut CompTypes,
) -> String {
    let start_str = if start { "start " } else { "" };
    let fn_name = fn_name_to_string(fn_name);
    let args: Vec<_> = f
        .args
        .iter()
        .map(|(x, _)| {
            let ident = local_name_to_string(x);
            let ty = ptype_to_string(f.locals.index_at(x), comptypes);

            format!("{ident}: {ty}")
        })
        .collect();
    let args = args.join(", ");

    let mut ret_ty = String::from("none");
    if let Some((ret, _)) = f.ret {
        ret_ty = ptype_to_string(f.locals.index_at(ret), comptypes);
    }
    let mut out = format!("{start_str}fn {fn_name}({args}) -> {ret_ty} {{\n");

    // fmt locals
    let mut locals: Vec<_> = f.locals.keys().collect();
    locals.sort_by_key(|l| l.0.get_internal());
    for l in locals {
        let ty = f.locals.index_at(l);
        let local = local_name_to_string(l);
        let ptype = ptype_to_string(ty, comptypes);
        out += &format!("  let {local}: {ptype};\n");
    }

    // blocks are formatted in order.
    let mut blocks: Vec<(BbName, BasicBlock)> = f.blocks.iter().collect();
    blocks.sort_by_key(|(BbName(name), _block)| *name);
    for (bb_name, bb) in blocks {
        let start = f.start == bb_name;
        out += &fmt_bb(bb_name, bb, start, comptypes);
    }
    out += "}\n\n";

    out
}


fn fmt_bb(
    bb_name: BbName,
    bb: BasicBlock,
    start: bool,
    comptypes: &mut CompTypes,
) -> String {
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

fn fmt_statement(st: Statement, comptypes: &mut CompTypes) -> String {
    match st {
        Statement::Assign {
            destination,
            source,
        } => {
            let left = place_expr_to_string(destination, comptypes);
            let right = value_expr_to_string(source, comptypes);
            format!("    {left} = {right};")
        },
        Statement::Finalize { place, fn_entry } => {
            let place = place_expr_to_string(place, comptypes);
            format!("    Finalize({place}, {fn_entry});")
        },
        Statement::StorageLive(local) => {
            let local = local_name_to_string(local);
            format!("    StorageLive({local});")
        }
        Statement::StorageDead(local) => {
            let local = local_name_to_string(local);
            format!("    StorageDead({local});")
        }
    }
}

fn fmt_call(
    callee: &str,
    arguments: List<ValueExpr>,
    ret: Option<PlaceExpr>,
    next_block: Option<BbName>,
    comptypes: &mut CompTypes,
) -> String {
    let args: Vec<_> = arguments
        .iter()
        .map(|x| value_expr_to_string(x, comptypes))
        .collect();
    let args = args.join(", ");

    let mut r = String::from("none");
    if let Some(ret) = ret {
        r = place_expr_to_string(ret, comptypes);
    }
    let mut next = String::new();
    if let Some(next_block) = next_block {
        let next_str = bb_name_to_string(next_block);
        next = format!(" -> {next_str}");
    }

    format!("    {r} = {callee}({args}){next};")
}

fn fmt_terminator(t: Terminator, comptypes: &mut CompTypes) -> String {
    match t {
        Terminator::Goto(bb) => {
            let bb = bb_name_to_string(bb);
            format!("    goto -> {bb};")
        }
        Terminator::If {
            condition,
            then_block,
            else_block,
        } => {
            let branch_expr = value_expr_to_string(condition, comptypes);
            let then_bb = bb_name_to_string(then_block);
            let else_bb = bb_name_to_string(else_block);
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
            let callee = value_expr_to_string(callee, comptypes);
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

pub fn bb_name_to_string(bb: BbName) -> String {
    let id = bb.0.get_internal();
    format!("bb{id}")
}

pub fn fn_name_to_string(fn_name: FnName) -> String {
    let id = fn_name.0.get_internal();
    format!("f{id}")
}

pub fn comptype_to_string(comptype_index: usize) -> String {
    format!("T{comptype_index}")
}

