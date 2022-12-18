#![feature(let_chains)]

// TODO consistent module naming scheme for module and entry function.
mod let_else;
mod imports;
mod argmatch;
mod merge_impls;
mod source;
mod typerec;
mod auto_derive;
mod auto_obj_bound;
mod index;
mod gccompat_impl;


use std::fs;
use std::path::{PathBuf, Path};
use std::process::Command;

pub mod prelude {
    pub use crate::source::Module;
    pub use std::collections::HashSet;
    pub use quote::{quote, format_ident, ToTokens};
    pub use syn::*;
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
    // setup "gen-minirust" directory.
    if !exists("../minirust") {
        eprintln!("You need to be in the `specr-transpile` folder in order to run it.!");
        eprintln!("Further `minirust` needs to be added to the repository root (for example by using `./clone-minirust.sh`)");
        std::process::exit(1);
    }

    mkdir("../gen-minirust");
    mkdir("../gen-minirust/src");

    let mods = source::fetch("../minirust");
    create_cargo_toml();
    create_lib(&mods);
    compile(mods);

    Command::new("cargo")
        .args(&["fmt", "--manifest-path", "../gen-minirust/Cargo.toml"])
        .output()
        .unwrap();
}

fn create_cargo_toml() {
    let toml = "[package]\n\
                name = \"gen-minirust\"\n\
                version = \"0.1.0\"\n\
                edition = \"2021\"\n\
                \n\
                [dependencies]\n\
                libspecr = { path = \"../libspecr\" }
               ";
    fs::write("../gen-minirust/Cargo.toml", &toml).unwrap();
}

fn create_lib(mods: &[Module]) {
    let mods: Vec<Ident> = mods.iter().map(|x| format_ident!("{}", x.name)).collect();
    let code = quote! {
        #![recursion_limit = "256"]
        #![feature(yeet_expr)]
        #![feature(never_type)]
        #![feature(iterator_try_collect)]
        #![feature(is_some_and)]
        #[allow(unused_imports)]
        #[macro_use] pub extern crate libspecr as specr;
        #( #[allow(unused_imports)] #[macro_use] pub mod #mods; )*
    };
    let code = code.to_string();
    fs::write("../gen-minirust/src/lib.rs", &code).unwrap();
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
        let ast = auto_derive::auto_derive(ast);
        let ast = index::index(ast);
        let ast = gccompat_impl::gccompat_impl(ast);
        // auto_obj_bound needs to be after gccompat_impl so that the impls are generated with coorect bounds.
        let ast = auto_obj_bound::auto_obj_bound(ast);

        // write AST back to Rust file.
        let code = ast.into_token_stream().to_string();
        let filename = format!("{}.rs", m.name);
        let p: PathBuf = ["..", "gen-minirust", "src", &filename].iter().collect();
        fs::write(&p, &code).unwrap();
    }
}
