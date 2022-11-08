#![feature(let_else)]

mod cp;

// TODO consistent module naming scheme for module and entry function.
mod imports;
mod argmatch;
mod merge_impls;
mod source;
mod typerec;
mod ret;

use std::fs;
use std::path::PathBuf;
use quote::ToTokens;
use std::process::Command;

use source::Module;

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

    let mods = source::fetch("minirust");
    compile(mods);

    let cargo_toml: PathBuf = ["generated", "Cargo.toml"].iter().collect();
    Command::new("cargo")
        .args(&["fmt", "--manifest-path", cargo_toml.to_str().unwrap()])
        .output()
        .unwrap();
}

fn compile(mods: Vec<Module>) {
    let mods = imports::add_imports(mods);
    let mods = typerec::typerec(mods);

    for m in mods.into_iter() {
        // apply all other compilation stages.
        let ast = argmatch::argmatch(m.ast);
        let ast = merge_impls::merge(ast);
        let ast = ret::add_ret(ast);

        // write AST back to Rust file.
        let code = ast.into_token_stream().to_string();
        let filename = format!("{}.rs", m.name);
        let p: PathBuf = ["generated", "src", &filename].iter().collect();
        fs::write(&p, &code).unwrap();
    }
}
