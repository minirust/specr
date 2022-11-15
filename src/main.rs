#![feature(let_else)]

// TODO consistent module naming scheme for module and entry function.
mod cp;
mod imports;
mod argmatch;
mod merge_impls;
mod source;
mod typerec;
mod ret;
mod autocopy;
mod gccompat_impl;

use std::fs;
use std::path::PathBuf;
use quote::ToTokens;
use std::process::Command;

use source::Module;

fn exists(s: &str) -> bool {
    std::path::Path::new(s).exists()
}

fn mkdir(name: &str) {
    if !exists(name) {
        let err_str = format!("Could not create directory \"{}\"", name);
        fs::create_dir(name).expect(&err_str);
    }
}

fn main() {
    // setup "generated" directory.
    if !exists("minirust") {
        eprintln!("You need to be at the project root to run `specr-transpile`!");
        eprintln!("Further `minirust` needs to be added (for example by using `./clone-minirust.sh`)");
        std::process::exit(1);
    }

    mkdir("generated");
    mkdir("generated/src");

    cp::cp_dir("libspecr", "generated/src/specr").expect("Copying libspecr to generated failed!");

    let mods = source::fetch("minirust");
    create_cargo_toml();
    create_lib(&mods);
    compile(mods);

    Command::new("cargo")
        .args(&["fmt", "--manifest-path", "generated/Cargo.toml"])
        .output()
        .unwrap();
}

fn create_cargo_toml() {
    let toml = "[package]\n\
                name = \"generated\"\n\
                version = \"0.1.0\"\n\
                edition = \"2021\"\n\
                \n\
                [dependencies]\n\
                num-bigint = \"0.4\"\n\
                num-traits = \"0.2.15\"\n\
               ";
    fs::write("generated/Cargo.toml", &toml).unwrap();
}

fn create_lib(mods: &[Module]) {
    let code = "#![feature(let_else)]\n\
                #![feature(try_trait_v2)]\n\
                #![feature(try_trait_v2_yeet)]\n\
                #![feature(try_trait_v2_residual)]\n\
                #![feature(yeet_expr)]\n\
                #![feature(iterator_try_collect)]\n\
                #![feature(never_type)]\n\
                #![feature(decl_macro)]\n\
                #![feature(map_try_insert)]\n\
                #![allow(unused)]\n\
                \n\
                #[macro_use] pub mod specr;\n\
               ";
    let mut code = String::from(code);
    for m in mods {
        code.push_str(&format!("#[macro_use] pub mod {};", m.name));
    }
    fs::write("generated/src/lib.rs", &code).unwrap();
}

fn compile(mods: Vec<Module>) {
    let mods = imports::add_imports(mods);
    // argmatch needs to be before typerec, as argmatch generates new match blocks!
    let mods = argmatch::argmatch(mods);
    let mods = typerec::typerec(mods);

    for m in mods.into_iter() {
        // apply all other compilation stages.
        let ast = merge_impls::merge(m.ast);
        let ast = ret::add_ret(ast);
        let ast = autocopy::autocopy(ast);
        let ast = gccompat_impl::gccompat_impl(ast);

        // write AST back to Rust file.
        let code = ast.into_token_stream().to_string();
        let filename = format!("{}.rs", m.name);
        let p: PathBuf = ["generated", "src", &filename].iter().collect();
        fs::write(&p, &code).unwrap();
    }
}
