// TODO consistent module naming scheme for module and entry function.
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

fn exists<T: AsRef<Path>>(t: T) -> bool {
    t.as_ref().exists()
}

fn mkdir<T: AsRef<Path>>(t: T) {
    let name = t.as_ref();
    if !exists(name) {
        let err_str = format!("Could not create directory \"{}\"", name.to_string_lossy().as_ref());
        fs::create_dir(name).expect(&err_str);
    }
}

fn main() {
    let config = Config::load();

    if !exists(&config.input_path()) {
        eprintln!("Input `{}` not found!", &config.input);
        std::process::exit(1);
    }

    mkdir(&config.output_path());
    mkdir(&config.output_path().join("src"));

    let mods = source::fetch(&config.input_path());
    create_cargo_toml(&config);
    create_rust_toolchain(&config);
    create_lib(&mods, &config);
    compile(mods, &config);
}

fn create_cargo_toml(config: &Config) {
    let package_name = &config.name;
    let toml = format!("[package]\n\
                name = \"{}\"\n\
                version = \"0.1.0\"\n\
                edition = \"2021\"\n\
                \n\
                [dependencies]\n\
                libspecr = \"=0.1.15\"\n\
                gccompat-derive = \"=0.1.1\"\n\
               ", package_name);
    fs::write(config.output_path().join("Cargo.toml"), &toml).unwrap();
}

fn create_rust_toolchain(config: &Config) {
    let Some(ref channel) = config.channel else { return };
    let toml = format!("[toolchain]\nchannel = \"{channel}\"");
    fs::write(config.output_path().join("rust-toolchain.toml"), &toml).unwrap();
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
    let code = parse_str::<syn::File>(&code.to_string()).unwrap();
    let code = prettyplease::unparse(&code);
    fs::write(config.output_path().join("src").join("lib.rs"), &code).unwrap();
}

fn compile(mods: Vec<Module>, config: &Config) {
    // argmatch needs to be before typerec, as argmatch generates new match blocks!
    let mods = argmatch::argmatch(mods);
    let mods = typerec::typerec(mods);

    for m in mods.into_iter() {
        // apply all other compilation stages.
        let ast = merge_impls::merge(m.ast);
        let ast = auto_derive::auto_derive(ast);
        let ast = index::index(ast);
        let ast = auto_obj_bound::auto_obj_bound(ast);

        // write AST back to Rust file.
        let code = prettyplease::unparse(&ast);
        let filename = format!("{}.rs", m.name);
        let p: PathBuf = config.output_path().join("src").join(filename);
        fs::write(&p, &code).unwrap();
    }
}
