/// This module gets the source code of MiniRust.

use std::fs;

pub struct Module {
    pub name: String,
    pub ast: syn::File,
}

// returns None if the module doesn't contain any source code.
fn mk_mod(basename: &str, modname: &str) -> Option<Module> {
    let mut code = String::new();
    let dirname = format!("{basename}/{modname}");

    for f in fs::read_dir(&dirname).unwrap() {
        let f = f.unwrap();
        let ty = f.file_type().unwrap();
        if !ty.is_file() { continue; }

        let name = f.file_name().into_string().unwrap();
        let name = format!("{dirname}/{name}");

        let fcode = fs::read_to_string(name).unwrap();
        let fcode = filter_pseudo_rust(&*fcode);
        code.push_str(&*fcode);
    }

    if code.is_empty() { return None; }

    // TODO implement graceful error messages using `syn` somehow.
    let ast = syn::parse_str::<syn::File>(&*code).unwrap_or_else(|e| {
        println!("parse error:");
        let start = e.span().start().line;
        let start = start.checked_sub(2).unwrap_or(0);
        let end = e.span().end().line + 2;
        for x in code.lines().skip(start).take(end-start) {
            println!("{}", x);
        }
        panic!("{}", &e)
    });
    Some(Module {
        name: modname.to_string(),
        ast
    })
}

pub fn fetch(folder: &str) -> Vec<Module> {
    let mut mods = Vec::new();

    // create the modules
    for d in fs::read_dir(folder).unwrap() {
        let d = d.unwrap();
        let ty = d.file_type().unwrap();
        if ty.is_dir() {
            let name = d.file_name().into_string().unwrap();

            // TODO find a less error-prone way to iterate over modules.
            if name == ".git" { continue; }

            if let Some(m) = mk_mod(folder, &*name) {
                mods.push(m);
            }
        }
    }

    // move prelude to the beginning to get macros to work.
    // TODO there needs to be a better solution.
    let i = mods.iter().position(|x| x.name == "prelude").unwrap();
    mods.swap(0, i);

    mods
}

// this filters out the code blocks ```rust <code> ```
// it will ignore ```rust,ignore <code> ``` blocks
fn filter_pseudo_rust(mut s: &str) -> String {
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
