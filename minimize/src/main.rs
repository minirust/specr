#![feature(rustc_private)]
#![feature(box_patterns)]
#![feature(never_type)]

// This is required since `get::Cb` contained `Option<Program>`.
#![recursion_limit = "256"]

extern crate rustc_hir;
extern crate rustc_target;
extern crate rustc_interface;
extern crate rustc_driver;
extern crate rustc_middle;
extern crate rustc_mir_dataflow;

mod rs {
    pub use rustc_hir::def_id::DefId;
    pub use rustc_target::abi::{Size, Align};
    pub use rustc_middle::mir::*;
    pub use rustc_middle::ty::*;
    pub use rustc_mir_dataflow::storage::always_storage_live_locals;
}

extern crate gen_minirust;
extern crate minisyntax;

pub use gen_minirust::lang::*;
pub use gen_minirust::mem::*;
pub use gen_minirust::prelude::*;

pub use gen_minirust::libspecr::*;
pub use gen_minirust::libspecr::prelude::*;
pub use gen_minirust::libspecr::hidden::*;

pub use std::format;
pub use std::string::String;
pub use gen_minirust::prelude::NdResult;

pub use minisyntax::run::*;
pub use minisyntax::dump::dump_program;

mod program;
use program::*;

mod ty;
use ty::*;

mod bb;
use bb::*;

mod rvalue;
use rvalue::*;

mod get;
use get::get_mini;

mod chunks;
use chunks::calc_chunks;

use std::collections::HashMap;
use std::path::Path;

fn main() {
    let file = std::env::args().skip(1)
                               .filter(|x| !x.starts_with('-'))
                               .next()
                               .unwrap_or_else(|| String::from("file.rs"));

    get_mini(file, |prog| {
        let dump = std::env::args().skip(1).any(|x| x == "--dump");
        if dump {
            dump_program(&prog);
        } else {
            match run_program(prog) {
                Outcome::IllFormed => eprintln!("ERR: program not well-formed."),
                Outcome::Stop => { /* silent exit. */ },
                Outcome::Ub(err) => eprintln!("UB: {}", err),
            }
        }
    });
}
