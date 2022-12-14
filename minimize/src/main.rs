#![feature(rustc_private)]
#![feature(box_patterns)]
#![feature(never_type)]

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
use run::*;

mod get;
use get::get_mini;

#[cfg(test)]
mod test;

use std::collections::HashMap;
use std::path::Path;

fn main() {
    let file = std::env::args().skip(1)
                               .filter(|x| !x.starts_with('-'))
                               .next()
                               .unwrap_or_else(|| String::from("file.rs"));
    let prog = get_mini(file);

    let dump = std::env::args().skip(1).any(|x| x == "--dump");
    if dump {
        dump_program(&prog);
    } else {
        match run_program(prog) {
            Outcome::Unwell => eprintln!("ERR: program not well-formed."),
            Outcome::Stop => { /* silent exit. */ },
            Outcome::Ub(err) => eprintln!("UB: {}", err),
        }
    }
}
