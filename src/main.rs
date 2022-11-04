#![feature(let_else)]

mod cp;
mod source;
mod argmatch;
mod access;
mod mac;
mod merge_impls;
mod clear_verbatim;
mod typerec;
mod ret;

use std::fs;
use std::path::PathBuf;
use quote::ToTokens;
use std::process::Command;
use reqwest::blocking::Client;

fn main() {
    let template_p = PathBuf::from("template");
    let generated_p = PathBuf::from("generated");
    cp::cp_dir(template_p, generated_p).expect("copying template failed!");

    let client = Client::new();

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
        modcode.push_str("use crate::baselib::prelude::*;\n");
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
		let ast = typerec::fix(ast);
		let ast = ret::add_ret(ast);

		let code = ast.into_token_stream().to_string();
        modcode.push_str(&code);
	}

    let filename = format!("{}.rs", modname);
    let p: PathBuf = ["generated", "src", &filename].iter().collect();
    fs::write(&p, &modcode).unwrap();
}
