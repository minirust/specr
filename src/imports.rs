use crate::Module;

use std::io::Write;

pub fn add_imports(mut mods: Vec<Module>) -> Vec<Module> {
    let names: Vec<&str> = mods.iter().map(|m| &*m.name).collect();
    let modimport_str = names.join(", ");
    let modimport_str = format!("use crate::{{{modimport_str}}};\n");

    // add imports within module
    for m in mods.iter_mut() {
        let mut imports = vec![
            "use crate::specr::prelude::*;\n",
            "use crate::specr;\n",
            &modimport_str,
        ];
        if m.name != "prelude" {
            imports.push("use crate::prelude::*;\n");
        }

        for (i, import) in imports.iter().enumerate() {
            let item = syn::parse_str::<syn::Item>(import).unwrap();
            m.ast.items.insert(i, item);
        }

    }

    // add modules to lib.rs
    let lib = format!("generated/src/lib.rs");
    let mut lib = std::fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(&lib)
        .unwrap();

    for m in mods.iter() {
        let line = format!("#[macro_use] pub mod {};\n", m.name);
        write!(lib, "{}", line).unwrap();
    }

    mods
}
