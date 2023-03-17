use crate::prelude::*;

use std::collections::HashMap;

// see https://rustc-dev-guide.rust-lang.org/name-resolution.html

// see https://doc.rust-lang.org/nightly/nightly-rustc/rustc_hir/def/enum.Namespace.html
#[derive(Hash, PartialEq, Eq, Debug, Clone, Copy)]
enum Namespace {
    Type, // also contains `mod` and `crate`s
    Value,
    Macro,
}

impl Namespace {
    fn fmt(self) -> &'static str {
        match self {
            Namespace::Type => "type",
            Namespace::Value => "value",
            Namespace::Macro => "macro",
        }
    }
}

type Entry = (Namespace, Ident);

#[derive(Default)]
struct NameTable(HashMap<Entry, NameTable>);

impl NameTable {
    fn build(mods: &[Module]) -> Self {
        let mut t = Self::default();
        for m in mods {
            let subtable = Self::build_by_ast(&m.ast);
            let entry = (Namespace::Type, format_ident!("{}", &m.name));
            t.0.insert(entry, subtable);
        }

        t
    }

    fn build_by_ast(ast: &syn::File) -> Self {
        let mut t = Self::default();
        for i in &ast.items {
            match i {
                Item::Enum(ie) => {
                    let mut variants_table = Self::default();
                    for v in &ie.variants {
                        let entry = (Namespace::Value, v.ident.clone());
                        variants_table.0.insert(entry, Self::default());
                    }
                    let entry = (Namespace::Type, ie.ident.clone());
                    // TODO throw error, if an equivalent entry already exists!
                    t.0.insert(entry, variants_table);
                }
                _ => {},
            }
        }

        t
    }

    fn fmt_impl(&self, depth: usize) -> String {
        let offset = " ".repeat(depth);
        let offset = &*offset;

        let mut s = String::new();
        for ((namespace, ident), subtable) in self.0.iter() {
            let substr = &*subtable.fmt_impl(depth+1);
            s.push_str(&format!("{}{} {}:\n{}", offset, namespace.fmt(), ident, substr));
        }
        s
    }

    fn fmt(&self) -> String {
        format!("NameTable [\n{}]", self.fmt_impl(0))
    }
}

pub fn nameres(mods: Vec<Module>) -> Vec<Module> {
    let table = NameTable::build(&mods);
    println!("{}", table.fmt());
    mods
}
