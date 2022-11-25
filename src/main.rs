#![feature(rustc_private)]
#![feature(box_patterns)]
#![feature(let_else)]

// is this the rustc_smir from our dependencies, or from the rustc-dev component?
// It's from the deps, as removing the dependency from Cargo.toml gives a compiler error.
extern crate rustc_smir;
use rustc_smir::very_unstable::interface::Queries;
use rustc_smir::very_unstable::interface::interface::Compiler;
use rustc_smir::very_unstable::driver::{Callbacks, RunCompiler, Compilation};

extern crate rustc_target;
mod mir {
    pub use rustc_smir::very_unstable::hir::def_id::DefId;
    pub use rustc_smir::very_unstable::middle::ty::TyCtxt;
    pub use rustc_smir::very_unstable::middle::mir::interpret::*;
    pub use rustc_smir::mir::*;
    pub use rustc_smir::ty::*;
    pub use rustc_target::abi::{Size, Align};
}

extern crate minirust_gen;

mod mini {
    pub use minirust_gen::lang::*;
    pub use minirust_gen::prelude::*;
}

mod specr {
    pub use minirust_gen::specr::*;
    pub use minirust_gen::specr::prelude::*;
}

mod program;
use program::*;

mod ty;
use ty::translate_ty;

mod bb;
use bb::translate_bb;

mod dump;
use dump::dump_program;

use std::collections::HashMap;

struct Cb;

impl Callbacks for Cb {
    fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, queries: &'tcx Queries<'tcx>) -> Compilation {
        queries.global_ctxt().unwrap().take().enter(|arg| {
            let prog = translate_program(arg);
            dump_program(&prog);
        });

        Compilation::Stop
    }
}

fn sysroot() -> String {
    let sysroot = std::process::Command::new("rustc")
        .arg("--print=sysroot")
        .current_dir(".")
        .output()
        .unwrap();

    std::str::from_utf8(&sysroot.stdout)
        .unwrap()
        .trim()
        .to_string()
}

fn main() {
    let args = [
        ".".to_string(),
        "file.rs".to_string(),
        "--sysroot".to_string(),
        sysroot(),
    ];
    RunCompiler::new(&args, &mut Cb).run().unwrap();
}
