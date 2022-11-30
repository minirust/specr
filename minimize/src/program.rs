use crate::*;

pub fn translate_program<'tcx>(tcx: rs::TyCtxt<'tcx>) -> mini::Program {
    let mut fnname_map: HashMap<rs::DefId, mini::FnName> = HashMap::new();

    for id in tcx.mir_keys(()) {
        let id = id.to_def_id();

        let fnname = fnname_map.len(); // .len() is the next free index
        let fnname = mini::FnName(specr::Name(fnname as u32));
        fnname_map.insert(id, fnname);
    }

    let (entry, _ty) = tcx.entry_fn(()).unwrap();
    let start = fnname_map[&entry];

    let mut program = mini::Program {
        start,
        functions: Default::default(),
    };

    for (id, fnname) in &fnname_map {
        let body = tcx.optimized_mir(id);
        let f = translate_body(body, &fnname_map, tcx);
        program.functions.insert(*fnname, f);
    }

    program

}

/// contains read-only data regarding the current function.
#[derive(Clone, Copy)]
pub struct FnCtxt<'fcx, 'tcx> {
    pub localname_map: &'fcx HashMap<rs::Local, mini::LocalName>,
    pub bbname_map: &'fcx HashMap<rs::BasicBlock, mini::BbName>,
    pub fnname_map: &'fcx HashMap<rs::DefId, mini::FnName>,
    pub tcx: rs::TyCtxt<'tcx>,
    pub body: &'tcx rs::Body<'tcx>,
}

fn translate_body<'tcx>(body: &'tcx rs::Body<'tcx>, fnname_map: &HashMap<rs::DefId, mini::FnName>, tcx: rs::TyCtxt<'tcx>) -> mini::Function {
    // associate names for each mir BB.
    let mut bbname_map: HashMap<rs::BasicBlock, mini::BbName> = HashMap::new();
    for bb_id in body.basic_blocks().indices() {
        let bbname = bbname_map.len(); // .len() is the next free index
        let bbname = mini::BbName(specr::Name(bbname as u32));
        bbname_map.insert(bb_id, bbname);
    }

    // bb with id 0 is the start block:
    // see https://doc.rust-lang.org/stable/nightly-rustc/src/rustc_middle/mir/mod.rs.html#1014-1042
    let start = mini::BbName(specr::Name(0));

    // associate names for each mir Local.
    let mut localname_map: HashMap<rs::Local, mini::LocalName> = HashMap::new();
    for local_id in body.local_decls.indices() {
        let localname = localname_map.len(); // .len() is the next free index
        let localname = mini::LocalName(specr::Name(localname as u32));
        localname_map.insert(local_id, localname);
    }

    // convert mirs Local-types to minirust.
    let mut locals = specr::Map::default();
    for (id, localname) in &localname_map {
        let local_decl = &body.local_decls[*id];
        locals.insert(*localname, translate_local(local_decl, tcx));
    }

    let fcx = FnCtxt {
        localname_map: &localname_map,
        bbname_map: &bbname_map,
        fnname_map,
        tcx,
        body,
    };

    // convert mirs BBs to minirust.
    let mut blocks = specr::Map::default();
    for (id, bbname) in &bbname_map {
        let bb_data = &body.basic_blocks()[*id];
        blocks.insert(*bbname, translate_bb(bb_data, fcx));
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

fn translate_local<'tcx>(local: &rs::LocalDecl<'tcx>, tcx: rs::TyCtxt<'tcx>) -> mini::PlaceType {
    let ty = translate_ty(local.ty, tcx);

    // TODO is this `empty` ParamEnv correct? probably not.
    // The generic args of the function need to be in scope here.
    let a = rs::ParamEnv::empty().and(local.ty);
    let layout = tcx.layout_of(a).unwrap().layout;
    let align = layout.align().pref;
    let align = translate_align(align);

    mini::PlaceType { ty, align }
}

// TODO implement this when mini::ArgAbi is somewhat complete.
pub fn arg_abi() -> mini::ArgAbi {
    mini::ArgAbi::Register
}

pub fn translate_align(align: rs::Align) -> specr::Align {
    specr::Align::from_bytes(align.bytes())
}
