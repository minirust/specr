use crate::*;

pub fn translate_program<'tcx>(tcx: rs::TyCtxt<'tcx>) -> mini::Program {
    let mut fname_map: HashMap<(rs::DefId, rs::SubstsRef<'tcx>), mini::FnName> = HashMap::new();
    let mut fmap: specr::Map<mini::FnName, mini::Function> = specr::Map::new();

    let (entry, _ty) = tcx.entry_fn(()).unwrap();
    let substs_ref: rs::SubstsRef<'tcx> = tcx.intern_substs(&[]);
    let start = mini::FnName(specr::Name(0));

    fname_map.insert((entry, substs_ref), start);

    // take any not-yet-implemented function:
    while let Some(fname) = fname_map.values().find(|k| !fmap.contains_key(**k)).copied() {
        let (def_id, substs_ref) = fname_map.iter()
                                            .find(|(_, f)| **f == fname)
                                            .map(|(r, _)| r)
                                            .unwrap();
        let body = tcx.optimized_mir(def_id);
        let body = tcx.subst_and_normalize_erasing_regions(substs_ref, rs::ParamEnv::empty(), body.clone());

        let is_start = *def_id == entry;
        let f = translate_body(body, is_start, &mut fname_map, tcx);
        fmap.insert(fname, f);
    }

    mini::Program {
        start,
        functions: fmap,
    }
}

/// contains read-only data regarding the current function.
pub struct FnCtxt<'tcx> {
    pub localname_map: HashMap<rs::Local, mini::LocalName>,
    pub bbname_map: HashMap<rs::BasicBlock, mini::BbName>,
    pub fnname_map: HashMap<(rs::DefId, rs::SubstsRef<'tcx>), mini::FnName>,
    pub tcx: rs::TyCtxt<'tcx>,
    pub body: rs::Body<'tcx>,
}

fn translate_body<'tcx>(body: rs::Body<'tcx>, is_start: bool, fnname_map_arg: &mut HashMap<(rs::DefId, rs::SubstsRef<'tcx>), mini::FnName>, tcx: rs::TyCtxt<'tcx>) -> mini::Function {
    let mut fnname_map = Default::default();
    std::mem::swap(&mut fnname_map, fnname_map_arg);

    // associate names for each mir BB.
    let mut bbname_map: HashMap<rs::BasicBlock, mini::BbName> = HashMap::new();
    for bb_id in body.basic_blocks.indices() {
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

    // TODO fix preventable clones.
    let mut fcx = FnCtxt {
        localname_map,
        bbname_map: bbname_map.clone(),
        fnname_map,
        tcx,
        body: body.clone(),
    };

    // convert mirs BBs to minirust.
    let mut blocks = specr::Map::default();
    for (id, bbname) in bbname_map.clone() {
        let bb_data = &body.basic_blocks[id];
        blocks.insert(bbname, translate_bb(bb_data, &mut fcx));
    }

    // "The first local is the return value pointer, followed by arg_count locals for the function arguments, followed by any user-declared variables and temporaries."
    // - https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/struct.Body.html
    let ret = match is_start {
        false => Some((mini::LocalName(specr::Name(0)), arg_abi())),
        // the start function has no `ret`.
        true => None,
    };

    let mut args = specr::List::default();
    for i in 0..fcx.body.arg_count {
        let i = i+1; // this starts counting with 1, as id 0 is the return value of the function.
        let localname = mini::LocalName(specr::Name(i as _));
        args.push((localname, arg_abi()));
    }

    let mut fnname_map = fcx.fnname_map;

    std::mem::swap(&mut fnname_map, fnname_map_arg);

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
