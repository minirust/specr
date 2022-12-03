// This module generates the Mir, and then calls `translate_program` to obtain the `mini::Program`.

use crate::*;
use rustc_interface::{Queries, interface::Compiler};
use rustc_driver::{RunCompiler, Compilation, Callbacks};

pub fn get_mini() -> mini::Program {
    if !std::path::Path::new("file.rs").exists() {
        eprintln!("You need to define some `file.rs` in order to run `minimize`.");
        std::process::exit(1);
    }

    let args = [
        ".".to_string(),
        "file.rs".to_string(),
        "--sysroot".to_string(),
        sysroot(),

        // flags taken from miŕi (see https://github.com/rust-lang/miri/blob/master/src/lib.rs#L116)
        "-Zalways-encode-mir".to_string(),
        "-Zmir-emit-retag".to_string(),
        "-Zmir-opt-level=0".to_string(),
        "--cfg=miri".to_string(),
        "-Cdebug-assertions=on".to_string(),

        // FIXME this one doesn't work as of now.
        // "-Zextra-const-ub-checks".to_string(),
    ];
    let mut cb = Cb(None);
    RunCompiler::new(&args, &mut cb).run().unwrap();
    cb.0.unwrap()
}

struct Cb(Option<mini::Program>);

impl Callbacks for Cb {
    fn after_analysis<'tcx>(&mut self, _compiler: &Compiler, queries: &'tcx Queries<'tcx>) -> Compilation {
        queries.global_ctxt().unwrap().take().enter(|arg| {
            let prog = translate_program(arg);
            self.0 = Some(prog);
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
