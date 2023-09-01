/// This module gets the source code of MiniRust.

use std::fs;
use std::path::{Path, PathBuf};

pub struct Module {
    pub name: String,
    pub ast: syn::File,
}

/// looks for subdirs in the directory `folder`, and converts them to a module.
pub fn fetch(folder: &Path) -> Vec<Module> {
    let mut mods = Vec::new();

    for d in fs::read_dir(folder).unwrap() {
        let d = d.unwrap();
        let ty = d.file_type().unwrap();
        if ty.is_dir() {
            let name = d.file_name().into_string().unwrap();

            // exclude ".git" from the module candidates.
            if name == ".git" { continue; }

            if let Some(m) = mk_mod(folder.to_string_lossy().as_ref(), &*name) {
                mods.push(m);
            }
        }
    }

    // move prelude to the beginning to get macros to work.
    let i = mods.iter().position(|x| x.name == "prelude").unwrap();
    mods.swap(0, i);

    mods
}

// returns None if the module doesn't contain any source code.
// TODO use Rusts Path API for this.
fn mk_mod(basename: &str, modname: &str) -> Option<Module> {
    let mut code = String::new();
    let dirname = PathBuf::from(format!("{basename}/{modname}"));

    let mut dirs = vec![dirname];
    while let Some(dir) = dirs.pop() {
        for f in fs::read_dir(&dir).unwrap() {
            let f = f.unwrap();
            let ty = f.file_type().unwrap();
            if ty.is_dir() {
                dirs.push(f.path());
                continue;
            }
            if !ty.is_file() { continue; }

            let name = f.file_name().into_string().unwrap();
            if !name.ends_with(".md") { continue; }

            let fcode = fs::read_to_string(f.path()).unwrap();
            let fcode = filter_specr_lang(&*fcode);
            code.push_str(&*fcode);
        }
    }

    if code.is_empty() { return None; }

    let ast = syn::parse_str::<syn::File>(&*code).unwrap_or_else(|e| {
        eprintln!("parse error:");
        let start = e.span().start().line;
        let start = start.checked_sub(2).unwrap_or(0);
        let end = e.span().end().line + 2;
        for x in code.lines().skip(start).take(end-start) {
            eprintln!("{}", x);
        }
        panic!("{}", &e)
    });
    Some(Module {
        name: modname.to_string(),
        ast
    })
}


// this filters out the code blocks ```rust <code> ```
// it will ignore ```rust,ignore <code> ``` blocks
fn filter_specr_lang(mut s: &str) -> String {
    const OFFSET1: usize = "\n```rust\n".len();
    const OFFSET2: usize = "\n```\n".len();

    let mut out = String::new();
    // note that this find(_) pattern doesn't match "```rust,ignore" due to the final newline.
    while let Some(i) = s.find("\n```rust\n") {
        s = &s[i+OFFSET1..];
        if let Some(j) = s.find("\n```\n") {
            out.push_str(&s[..j]);
            out.push_str("\n\n");
            s = &s[j+OFFSET2..];
        } else { panic!("unclosed code segment!"); }
    }

    out
}
