#![feature(let_else)]

mod source;
mod argmatch;
mod access;
mod mac;
mod merge_impls;
mod clear_verbatim;
mod ret;

use std::fs;
use std::path::{Path, PathBuf};
use quote::ToTokens;
use std::process::Command;
use reqwest::blocking::Client;

fn main() {
    let genpath = Path::new("generated");
    if genpath.exists() {
       fs::remove_dir_all(genpath).unwrap();
    }

    Command::new("cargo")
        .args(&["new", "generated", "--lib"])
        .output()
        .unwrap();

    let client = Client::new();

    let gen_baselib: PathBuf = ["generated", "src", "baselib.rs"].iter().collect();
    let baselib_code = include_str!("baselib.rs");
    fs::write(&gen_baselib, &baselib_code).unwrap();

    make_mod(&client, "prelude", &["prelude.md"]);

    // TODO automatically find files!
    make_mod(&client, "lang", &[
        "lang/machine.md",
        "lang/operator.md",
        "lang/prelude.md",
        "lang/step.md",
        "lang/syntax.md",
        "lang/types.md",
        "lang/values.md",
        "lang/well-formed.md"
    ]);
    make_mod(&client, "mem", &[
        "mem/basic.md",
        "mem/interface.md",
        "mem/intptrcast.md"
    ]);

    let lib_path: PathBuf = ["generated", "src", "lib.rs"].iter().collect();
	fs::write(&lib_path,
		"#![feature(let_else)]                \n\
		#![feature(try_trait_v2)]             \n\
		#![feature(try_trait_v2_yeet)]        \n\
		#![feature(yeet_expr)]                \n\
		#![feature(associated_type_defaults)] \n\
		#![allow(unused)]                     \n\
		pub mod baselib;                      \n\
		pub mod prelude;                      \n\
		pub mod lang;                         \n\
		pub mod mem;").unwrap();


    let cargo_toml: PathBuf = ["generated", "Cargo.toml"].iter().collect();
    Command::new("cargo")
        .args(&["fmt", "--manifest-path", cargo_toml.to_str().unwrap()])
        .output()
        .unwrap();
}

fn make_mod(client: &Client, modname: &str, filenames: &[&str]) {
    let mut modcode = String::new();
    if modname != "prelude" {
        modcode.push_str("use crate::prelude::*;\n");
    }

    for f in filenames {
        let code = source::fetch(client, f);
        let ast = syn::parse_str::<syn::File>(&code).unwrap_or_else(|_| panic!("Cannot parse code:\n{code}"));
        let ast = argmatch::argmatch(ast);
		let ast = clear_verbatim::clear_verbatim(ast);
		let ast = mac::add_macro_exports(ast);
		let ast = access::access(ast);
		let ast = merge_impls::merge(ast);
		let ast = clear_verbatim::clear_empty_impls(ast);
		let ast = ret::add_ret(ast);

		let code = ast.into_token_stream().to_string();
        modcode.push_str(&code);
	}

    let filename = format!("{}.rs", modname);
    let p: PathBuf = ["generated", "src", &filename].iter().collect();
    fs::write(&p, &modcode).unwrap();
}
