use crate::*;

/// maps Rust function calls to minirust FnNames.
pub type FnNameMap<'tcx> = HashMap<(rs::DefId, rs::SubstsRef<'tcx>), FnName>;

pub fn translate_program<'tcx>(tcx: rs::TyCtxt<'tcx>) -> Program {
    let mut fn_name_map = FnNameMap::new();
    let mut fmap: Map<FnName, Function> = Map::new();

    let (entry, _ty) = tcx.entry_fn(()).unwrap();
    let substs_ref: rs::SubstsRef<'tcx> = tcx.intern_substs(&[]);
    let entry_name = FnName(Name::new(0));

    fn_name_map.insert((entry, substs_ref), entry_name);

    // take any not-yet-implemented function:
    while let Some(fn_name) = fn_name_map.values().find(|k| !fmap.contains_key(**k)).copied() {
        let (def_id, substs_ref) = fn_name_map.iter()
                                            .find(|(_, f)| **f == fn_name)
                                            .map(|(r, _)| r)
                                            .unwrap();
        let body = tcx.optimized_mir(def_id);
        let body = tcx.subst_and_normalize_erasing_regions(substs_ref, rs::ParamEnv::empty(), body.clone());

        let (f, m) = translate_body(body, fn_name_map, tcx);
        fmap.insert(fn_name, f);
        fn_name_map = m;
    }

    let number_of_fns = fn_name_map.len();

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
    pub fn_name_map: FnNameMap<'tcx>,
    pub local_name_map: HashMap<rs::Local, LocalName>,
    pub bb_name_map: HashMap<rs::BasicBlock, BbName>,
    pub tcx: rs::TyCtxt<'tcx>,
    pub body: rs::Body<'tcx>,
}

/// translates a function body.
/// Any fn calls occuring during this translation will be added to the `FnNameMap`.
fn translate_body<'tcx>(body: rs::Body<'tcx>, fn_name_map: FnNameMap<'tcx>, tcx: rs::TyCtxt<'tcx>) -> (Function, FnNameMap<'tcx>) {
    // associate names for each mir BB.
    let mut bb_name_map: HashMap<rs::BasicBlock, BbName> = HashMap::new();
    for bb_id in body.basic_blocks.indices() {
        let bb_name = bb_name_map.len(); // .len() is the next free index
        let bb_name = BbName(Name::new(bb_name as u32));
        bb_name_map.insert(bb_id, bb_name);
    }

    // bb with id 0 is the start block:
    // see https://doc.rust-lang.org/stable/nightly-rustc/src/rustc_middle/mir/mod.rs.html#1014-1042
    let start = BbName(Name::new(0));

    // associate names for each mir Local.
    let mut local_name_map: HashMap<rs::Local, LocalName> = HashMap::new();
    for local_id in body.local_decls.indices() {
        let local_name = local_name_map.len(); // .len() is the next free index
        let local_name = LocalName(Name::new(local_name as u32));
        local_name_map.insert(local_id, local_name);
    }

    // convert mirs Local-types to minirust.
    let mut locals = Map::default();
    for (id, local_name) in &local_name_map {
        let local_decl = &body.local_decls[*id];
        locals.insert(*local_name, translate_local(local_decl, tcx));
    }

    let mut fcx = FnCtxt {
        local_name_map,
        bb_name_map: bb_name_map.clone(),
        fn_name_map,
        tcx,
        body: body.clone(),
    };

    // convert mirs BBs to minirust.
    let mut blocks = Map::default();
    for (id, bb_name) in bb_name_map {
        let bb_data = &body.basic_blocks[id];
        blocks.insert(bb_name, translate_bb(bb_data, &mut fcx));
    }

    // "The first local is the return value pointer, followed by arg_count locals for the function arguments, followed by any user-declared variables and temporaries."
    // - https://doc.rust-lang.org/stable/nightly-rustc/rustc_middle/mir/struct.Body.html
    let ret = Some((LocalName(Name::new(0)), arg_abi()));

    let mut args = List::default();
    for i in 0..fcx.body.arg_count {
        let i = i+1; // this starts counting with 1, as id 0 is the return value of the function.
        let local_name = LocalName(Name::new(i as _));
        args.push((local_name, arg_abi()));
    }

    let fn_name_map = fcx.fn_name_map;

    let f = Function {
        locals,
        args,
        ret,
        blocks,
        start
    };

    (f, fn_name_map)
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
