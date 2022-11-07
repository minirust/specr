/// This module gets the source code of MiniRust.

pub fn fetch(filename: &str) -> String {
    let filename = format!("minirust/{}", filename);
    let s = std::fs::read_to_string(filename).unwrap();

    filter_pseudo_rust(&s)
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
