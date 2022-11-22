use crate::*;

use std::collections::HashMap;

pub fn translate<'tcx>(tyc: mir::TyCtxt<'tcx>) -> mini::Program {
    let mut fname_map = HashMap::new();

    for id in tyc.mir_keys(()) {
        let id = id.to_def_id();

        let fname = fname_map.len();
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
    todo!()
}
