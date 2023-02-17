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

mod config;

use std::fs;
use std::path::{PathBuf, Path};
use std::process::Command;

pub mod prelude {
    pub use crate::source::Module;
    pub use crate::config::Config;

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
    let config = Config::load();

    if !exists(&config.input) {
        eprintln!("You need to be in the `specr-transpile` folder in order to run it.!");
        eprintln!("Further `{}` needs to be added to the repository root", config.input);
        std::process::exit(1);
    }

    mkdir(&config.output);
    mkdir(&format!("{}/src", config.output));

    let mods = source::fetch(&config.input);
    create_cargo_toml(&config);
    create_lib(&mods, &config);
    compile(mods, &config);

    Command::new("cargo")
        .args(&["fmt", "--manifest-path", &format!("{}/Cargo.toml", config.output)])
        .output()
        .unwrap();
}

fn create_cargo_toml(config: &Config) {
    let package_name = config.output.split("/").last().unwrap();
    let toml = format!("[package]\n\
                name = \"{}\"\n\
                version = \"0.1.0\"\n\
                edition = \"2021\"\n\
                \n\
                [dependencies]\n\
                libspecr = {{ path = \"../libspecr\" }}
                gccompat-derive = {{ path = \"../gccompat-derive\" }}
               ", package_name);
    fs::write(&format!("{}/Cargo.toml", config.output), &toml).unwrap();
}

fn create_lib(mods: &[Module], config: &Config) {
    let mods: Vec<Ident> = mods.iter().map(|x| format_ident!("{}", x.name)).collect();

    let attrs = parse_str::<syn::File>(&config.attrs.join("\n")).unwrap();

    let code = quote! {
        #attrs
        #[allow(unused_imports)]
        #[macro_use] pub extern crate libspecr;
        #( #[allow(unused_imports)] #[macro_use] pub mod #mods; )*
    };
    let code = code.to_string();
    fs::write(&format!("{}/src/lib.rs", config.output), &code).unwrap();
}

fn compile(mods: Vec<Module>, config: &Config) {
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
        let ast = auto_obj_bound::auto_obj_bound(ast);

        // write AST back to Rust file.
        let code = ast.into_token_stream().to_string();
        let filename = format!("{}.rs", m.name);
        let p: PathBuf = [&config.output, "src", &filename].iter().collect();
        fs::write(&p, &code).unwrap();
    }
}
