#![feature(never_type)]

extern crate gen_minirust;

pub use gen_minirust::lang::*;
pub use gen_minirust::mem::*;
pub use gen_minirust::prelude::*;

pub use gen_minirust::specr::*;
pub use gen_minirust::specr::prelude::*;
pub use gen_minirust::specr::hidden::*;

pub use std::format;
pub use std::string::String;
pub use gen_minirust::prelude::NdResult;

pub mod dump;
pub mod build;
pub mod run;
