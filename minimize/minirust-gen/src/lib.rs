#![feature(let_else)]
#![feature(try_trait_v2)]
#![feature(try_trait_v2_yeet)]
#![feature(try_trait_v2_residual)]
#![feature(yeet_expr)]
#![feature(iterator_try_collect)]
#![feature(never_type)]
#![feature(decl_macro)]
#![feature(map_try_insert)]
#![allow(unused)]
#[macro_use]
mod libspecr;
pub use libspecr::public as specr;
#[macro_use]
pub mod prelude;
#[macro_use]
pub mod lang;
#[macro_use]
pub mod mem;
