/// This module gets the source code of MiniRust.

pub fn fetch(filename: &str) -> String {
    let filename = format!("minirust/{}", filename);
    let s = std::fs::read_to_string(filename).unwrap();

    filter_pseudo_rust(&s)
}

fn filter_pseudo_rust(mut s: &str) -> String {
    const OFFSET1: usize = "\n```rust".len();
    const OFFSET2: usize = "\n```".len();

    let mut out = String::new();
    while let Some(i) = s.find("\n```rust") {
        s = &s[i+OFFSET1..];
        if let Some(j) = s.find("\n```") {
            out.push_str(&s[..j]);
            out.push_str("\n\n");
            s = &s[j+OFFSET2..];
        } else { panic!("unclosed code segment!"); }
    }

    out
}
