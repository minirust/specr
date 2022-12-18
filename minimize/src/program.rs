use crate::*;

pub fn translate_program<'tcx>(tcx: rs::TyCtxt<'tcx>) -> Program {
    let mut fname_map: HashMap<(rs::DefId, rs::SubstsRef<'tcx>), FnName> = HashMap::new();
    let mut fmap: Map<FnName, Function> = Map::new();

    let (entry, _ty) = tcx.entry_fn(()).unwrap();
    let substs_ref: rs::SubstsRef<'tcx> = tcx.intern_substs(&[]);
    let entry_name = FnName(Name::new(0));

    fname_map.insert((entry, substs_ref), entry_name);

    // take any not-yet-implemented function:
    while let Some(fname) = fname_map.values().find(|k| !fmap.contains_key(**k)).copied() {
        let (def_id, substs_ref) = fname_map.iter()
                                            .find(|(_, f)| **f == fname)
                                            .map(|(r, _)| r)
                                            .unwrap();
        let body = tcx.optimized_mir(def_id);
        let body = tcx.subst_and_normalize_erasing_regions(substs_ref, rs::ParamEnv::empty(), body.clone());

        let f = translate_body(body, &mut fname_map, tcx);
        fmap.insert(fname, f);
    }

    let number_of_fns = fname_map.len();

    // add a `start` function, which calls `entry`.
    let start = FnName(Name::new(number_of_fns as _));
    fmap.insert(start, mk_start_fn(entry_name));

    Program {
        start,
        functions: fmap,
    }
}

fn mk_start_fn(entry: FnName) -> Function {
    let b0_name = BbName(Name::new(0));
    let b1_name = BbName(Name::new(1));

    let b0 = BasicBlock {
        statements: List::new(),
        terminator: Terminator::Call {
            callee: entry,
            arguments: List::new(),
            ret: None,
            next_block: Some(b1_name),
        },
    };

    let b1 = BasicBlock {
        statements: List::new(),
        terminator: Terminator::CallIntrinsic {
            intrinsic: Intrinsic::Exit,
            arguments: List::new(),
            ret: None,
            next_block: None,
        },
    };

    let mut blocks = Map::new();
    blocks.insert(b0_name, b0);
    blocks.insert(b1_name, b1);

    Function {
        locals: Map::new(),
        args: List::new(),
        ret: None,
        blocks,
        start: b0_name,
    }
}

/// data regarding the currently translated function.
pub struct FnCtxt<'tcx> {
    /// This is the only field mutated during translation.
    /// Upon function call, the callees DefId + SubstsRef will be mapped to a fresh `FnName`.
    pub fnname_map: HashMap<(rs::DefId, rs::SubstsRef<'tcx>), FnName>,
    pub localname_map: HashMap<rs::Local, LocalName>,
    pub bbname_map: HashMap<rs::BasicBlock, BbName>,
    pub tcx: rs::TyCtxt<'tcx>,
    pub body: rs::Body<'tcx>,
}

// TODO implement non-mem::swap solution
fn translate_body<'tcx>(body: rs::Body<'tcx>, fnname_map_arg: &mut HashMap<(rs::DefId, rs::SubstsRef<'tcx>), FnName>, tcx: rs::TyCtxt<'tcx>) -> Function {
    let mut fnname_map = Default::default();
    std::mem::swap(&mut fnname_map, fnname_map_arg);

    // associate names for each mir BB.
    let mut bbname_map: HashMap<rs::BasicBlock, BbName> = HashMap::new();
    for bb_id in body.basic_blocks.indices() {
        let bbname = bbname_map.len(); // .len() is the next free index
        let bbname = BbName(Name::new(bbname as u32));
        bbname_map.insert(bb_id, bbname);
    }

    // bb with id 0 is the start block:
    // see https://doc.rust-lang.org/stable/nightly-rustc/src/rustc_middle/mir/mod.rs.html#1014-1042
    let start = BbName(Name::new(0));

    // associate names for each mir Local.
    let mut localname_map: HashMap<rs::Local, LocalName> = HashMap::new();
    for local_id in body.local_decls.indices() {
        let localname = localname_map.len(); // .len() is the next free index
        let localname = LocalName(Name::new(localname as u32));
        localname_map.insert(local_id, localname);
    }

    // convert mirs Local-types to minirust.
    let mut locals = Map::default();
    for (id, localname) in &localname_map {
        let local_decl = &body.local_decls[*id];
        locals.insert(*localname, translate_local(local_decl, tcx));
    }

    let mut fcx = FnCtxt {
        localname_map,
        bbname_map: bbname_map.clone(),
        fnname_map,
        tcx,
        body: body.clone(),
    };

    // convert mirs BBs to minirust.
    let mut blocks = Map::default();
    for (id, bbname) in bbname_map {
        let bb_data = &body.basic_blocks[id];
        blocks.insert(bbname, translate_bb(bb_data, &mut fcx));
    }

    // "The first local is the return value pointer, followed by arg_count locals for the function arguments, followed by any user-declared variables and temporaries."
    // - https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/struct.Body.html
    let ret = Some((LocalName(Name::new(0)), arg_abi()));

    let mut args = List::default();
    for i in 0..fcx.body.arg_count {
        let i = i+1; // this starts counting with 1, as id 0 is the return value of the function.
        let localname = LocalName(Name::new(i as _));
        args.push((localname, arg_abi()));
    }

    let mut fnname_map = fcx.fnname_map;

    std::mem::swap(&mut fnname_map, fnname_map_arg);

    Function {
        locals,
        args,
        ret,
        blocks,
        start
    }
}

fn translate_local<'tcx>(local: &rs::LocalDecl<'tcx>, tcx: rs::TyCtxt<'tcx>) -> PlaceType {
    let ty = translate_ty(local.ty, tcx);

    // generics have already been resolved before, so `ParamEnv::empty()` is correct.
    let a = rs::ParamEnv::empty().and(local.ty);
    let layout = tcx.layout_of(a).unwrap().layout;
    let align = layout.align().pref;
    let align = translate_align(align);

    PlaceType { ty, align }
}

// TODO implement this when ArgAbi is somewhat complete.
pub fn arg_abi() -> ArgAbi {
    ArgAbi::Register
}

pub fn translate_align(align: rs::Align) -> Align {
    Align::from_bytes(align.bytes())
}
