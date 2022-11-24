#![feature(let_chains)]

mod cp;

// TODO consistent module naming scheme for module and entry function.
mod let_else;
mod imports;
mod argmatch;
mod merge_impls;
mod source;
mod typerec;
mod ret;
mod autoattr;
mod autobounds;
mod index;
mod gccompat_impl;


use std::fs;
use std::path::{PathBuf, Path};
use std::process::Command;

use source::Module;

pub mod prelude {
    pub use crate::source::Module;
    pub use quote::{quote, format_ident, ToTokens};
    pub use syn::*;
    pub use syn::token::{Brace, Match};
    pub use syn::visit_mut::*;
    pub use syn::visit::*;
    pub use proc_macro2::{TokenStream, TokenTree, Span};
    pub use syn::punctuated::Punctuated;

}
use prelude::*;

fn exists(s: &str) -> bool {
    Path::new(s).exists()
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

    cp::cp_dir("libspecr", "generated/src/libspecr").expect("Copying libspecr to generated failed!");

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
                im = \"15.1.0\"\n\
               ";
    fs::write("generated/Cargo.toml", &toml).unwrap();
}

fn create_lib(mods: &[Module]) {
    let mods: Vec<Ident> = mods.iter().map(|x| format_ident!("{}", x.name)).collect();
    let code = quote! {
        #![feature(try_trait_v2)]
        #![feature(try_trait_v2_yeet)]
        #![feature(try_trait_v2_residual)]
        #![feature(yeet_expr)]
        #![feature(iterator_try_collect)]
        #![feature(never_type)]
        #![feature(decl_macro)]
        #![feature(map_try_insert)]
        #![allow(unused)]
        #[macro_use] mod libspecr;
        pub use libspecr::public as specr;
        #( #[macro_use] pub mod #mods; )*
    };
    let code = code.to_string();
    fs::write("generated/src/lib.rs", &code).unwrap();
}

fn compile(mods: Vec<Module>) {
    let mods = imports::add_imports(mods);
    // argmatch needs to be before typerec, as argmatch generates new match blocks!
    let mods = argmatch::argmatch(mods);
    let mods = typerec::typerec(mods);

    for m in mods.into_iter() {
        // apply all other compilation stages.
        let ast = let_else::let_else(m.ast);
        let ast = merge_impls::merge(ast);
        let ast = ret::add_ret(ast);
        let ast = autoattr::autoattr(ast);
        let ast = index::index(ast);
        let ast = gccompat_impl::gccompat_impl(ast);
        // autobounds needs to be after gccompat_impl so that the impls are generated with coorect bounds.
        let ast = autobounds::autobounds(ast);

        // write AST back to Rust file.
        let code = ast.into_token_stream().to_string();
        let filename = format!("{}.rs", m.name);
        let p: PathBuf = ["generated", "src", &filename].iter().collect();
        fs::write(&p, &code).unwrap();
    }
}
