use crate::*;

use std::collections::HashMap;

pub fn translate_program<'tcx>(tyc: mir::TyCtxt<'tcx>) -> mini::Program {
    let mut fname_map: HashMap<mir::DefId, mini::FnName> = HashMap::new();

    for id in tyc.mir_keys(()) {
        let id = id.to_def_id();

        let fname = fname_map.len(); // .len() is the next free index
        let fname = mini::FnName(specr::Name(fname as u32));
        fname_map.insert(id, fname);
    }

    let (entry, _ty) = tyc.entry_fn(()).unwrap();
    let start = fname_map[&entry];

    let mut program = mini::Program {
        start,
        functions: Default::default(),
    };

    for (id, fname) in &fname_map {
        let body = tyc.optimized_mir(id);
        let f = translate_body(body);
        program.functions.insert(*fname, f);
    }

    program

}

fn translate_body(body: &mir::Body) -> mini::Function {
    // associate names for each mir BB.
    let mut bbname_map: HashMap<mir::BasicBlock, mini::BbName> = HashMap::new();
    for bb_id in body.basic_blocks().indices() {
        let bbname = bbname_map.len(); // .len() is the next free index
        let bbname = mini::BbName(specr::Name(bbname as u32));
        bbname_map.insert(bb_id, bbname);
    }

    // bb with id 0 is the start block:
    // see https://doc.rust-lang.org/stable/nightly-rustc/src/rustc_middle/mir/mod.rs.html#1014-1042
    let start = mini::BbName(specr::Name(0));


    // associate names for each mir Local.
    let mut localname_map: HashMap<mir::Local, mini::LocalName> = HashMap::new();
    for local_id in body.local_decls.indices() {
        let localname = localname_map.len(); // .len() is the next free index
        let localname = mini::LocalName(specr::Name(localname as u32));
        localname_map.insert(local_id, localname);
    }

    // convert mirs Local-types to minirust.
    let mut locals = specr::Map::default();
    for (id, localname) in &localname_map {
        let local_decl = &body.local_decls[*id];
        locals.insert(*localname, translate_local(local_decl));
    }

    // convert mirs BBs to minirust.
    let mut blocks = specr::Map::default();
    for (id, bbname) in &bbname_map {
        let bb_data = &body.basic_blocks()[*id];
        blocks.insert(*bbname, translate_bb(bb_data));
    }

    // "The first local is the return value pointer, followed by arg_count locals for the function arguments, followed by any user-declared variables and temporaries."
    // - https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/struct.Body.html
    let ret = (mini::LocalName(specr::Name(0)), arg_abi());

    let mut args = specr::List::default();
    for i in 0..body.arg_count {
        let i = i+1; // this starts counting with 1, as id 0 is the return value of the function.
        let localname = mini::LocalName(specr::Name(i as _));
        args.push((localname, arg_abi()));
    }

    mini::Function {
        locals,
        args,
        ret,
        blocks,
        start
    }
}

fn translate_local(local: &mir::LocalDecl) -> mini::PlaceType {
    let ty = translate_ty(&local.ty);
    let align = align();

    mini::PlaceType { ty, align }
}

fn arg_abi() -> mini::ArgAbi {
    todo!()
}

fn align() -> mini::Align {
    todo!()
}
