#![feature(rustc_private)]
#![feature(box_patterns)]
#![feature(let_else)]

// This is required since `get::Cb` contained `Option<mini::Program>`.
#![recursion_limit = "256"]

extern crate rustc_hir;
extern crate rustc_target;
extern crate rustc_interface;
extern crate rustc_driver;
extern crate rustc_middle;

mod rs {
    pub use rustc_hir::def_id::DefId;
    pub use rustc_target::abi::{Size, Align};
    pub use rustc_middle::mir::*;
    pub use rustc_middle::ty::*;
}

extern crate gen_minirust;

mod mini {
    pub use gen_minirust::lang::*;
    pub use gen_minirust::mem::*;
    pub use gen_minirust::prelude::*;
}

mod specr {
    pub use gen_minirust::specr::*;
    pub use gen_minirust::specr::prelude::*;
    pub use gen_minirust::specr::hidden::*;
}

mod program;
use program::*;

mod ty;
use ty::*;

mod bb;
use bb::*;

mod rvalue;
use rvalue::*;

mod dump;
use dump::dump_program;

mod run;
use run::run_program;

mod get;
use get::get_mini;

use std::collections::HashMap;

fn main() {
    let prog = get_mini();

    let dump = std::env::args().any(|x| x == "--dump");
    if dump {
        dump_program(&prog);
    } else {
        run_program(prog);
    }
}
