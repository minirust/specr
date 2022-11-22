#![feature(rustc_private)]

// is this the rustc_smir from our dependencies, or from the rustc-dev component?
// It's from the deps, as removing the dependency from Cargo.toml gives a compiler error.
extern crate rustc_smir;
use rustc_smir::very_unstable::interface::Queries;
use rustc_smir::very_unstable::interface::interface::Compiler;
use rustc_smir::very_unstable::driver::{Callbacks, RunCompiler, Compilation};

mod mir {
    pub use rustc_smir::very_unstable::hir::def_id::DefId;
    pub use rustc_smir::very_unstable::middle::ty::TyCtxt;
    pub use rustc_smir::mir::*;
}

extern crate minirust_gen;
use minirust_gen::lang as mini;
use minirust_gen::specr as specr;


mod translate;
use translate::translate;

mod dump;
use dump::dump_program;

struct Cb;

impl Callbacks for Cb {
    fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, queries: &'tcx Queries<'tcx>) -> Compilation {
        queries.global_ctxt().unwrap().take().enter(|arg| {
            let prog = translate(arg);
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
