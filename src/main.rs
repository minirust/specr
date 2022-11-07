#![feature(let_else)]

mod cp;

// TODO consistent module naming scheme for module and entry function.
mod argmatch;
mod merge_impls;
mod baselib_use;
mod source;
mod access;
mod mac;
mod clear_verbatim;
mod typerec;
mod ret;

use std::fs;
use std::path::PathBuf;
use quote::ToTokens;
use std::process::Command;

fn exists(s: &str) -> bool {
    std::path::Path::new(s).exists()
}

fn main() {
    // setup "generated" directory.

    if !exists("template") {
        eprintln!("You need to be at the project root to run `specr`!");
        std::process::exit(1);
    }

    if !exists("generated") {
        fs::create_dir("generated").expect("Could not create \"generated\" directory.");
    }
    fs::copy("template/Cargo.toml", "generated/Cargo.toml").expect("Could not copy Cargo.toml");
    cp::cp_dir("template/src", "generated/src").expect("copying src failed!");

    // TODO automatically find files!
    compile(&[
        ("prelude", &["prelude.md"]),
        ("lang", &[
            "lang/machine.md",
            "lang/operator.md",
            "lang/prelude.md",
            "lang/step.md",
            "lang/syntax.md",
            "lang/types.md",
            "lang/values.md",
            "lang/well-formed.md"
        ]),
        ("mem", &[
            "mem/basic.md",
            "mem/interface.md",
            "mem/intptrcast.md"
        ])
    ]);

    let cargo_toml: PathBuf = ["generated", "Cargo.toml"].iter().collect();
    Command::new("cargo")
        .args(&["fmt", "--manifest-path", cargo_toml.to_str().unwrap()])
        .output()
        .unwrap();
}

fn compile(modfiles: &[(/*modname: */ &str, /*files: */ &[&str])]) {
    let modnames: Vec<&str> = modfiles.iter().cloned().map(|(x, _)| x).collect();

    let mut mods: Vec<syn::File> = Vec::new();
    for (modname, files) in modfiles.iter().cloned() {
        // add prelude imports
        let mut code = String::new();
        code.push_str("use crate::baselib::prelude::*;\n");
        code.push_str("use crate::hiddenlib::prelude::*;\n");
        code.push_str("use crate::{lang, mem, baselib, hiddenlib};\n");
        if modname != "prelude" {
            code.push_str("use crate::prelude::*;\n");
        }

        // merge all .md files into one rust file
        for f in files {
            code.push_str(&source::fetch(f));
        }

        let ast = syn::parse_str::<syn::File>(&code)
                    .unwrap_or_else(|_| panic!("Cannot parse code:\n{code}"));
        mods.push(ast);
    }

    // resolve infinite type recursion
    let mods = typerec::typerec(mods);

    for (modname, ast) in modnames.iter().zip(mods.into_iter()) {
        // apply all other compilation stages.
        let ast = access::access(ast);
        let ast = argmatch::argmatch(ast);
        let ast = merge_impls::merge(ast);
        let ast = baselib_use::apply_baselib_use(ast);
        let ast = clear_verbatim::clear_verbatim(ast);
        let ast = mac::add_macro_exports(ast);
        let ast = ret::add_ret(ast);

        // write AST back to Rust file.
        let code = ast.into_token_stream().to_string();
        let filename = format!("{}.rs", modname);
        let p: PathBuf = ["generated", "src", &filename].iter().collect();
        fs::write(&p, &code).unwrap();
    }
}
