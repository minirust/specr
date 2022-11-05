/// This module gets the source code of MiniRust.

use reqwest::blocking::Client;

pub fn fetch(client: &Client, filename: &str) -> String {
	let filename = format!("{}{}", "https://raw.githubusercontent.com/memoryleak47/minirust/dev/", filename);

	let s = client.get(filename)
		.send().expect("download failed!")
		.text().expect("Cannot convert HTTP Response to text!");

    filter_pseudo_rust(&s)
}

// for testing purposes only:
#[allow(unused)]
pub fn fetch_local(client: &Client, filename: &str) -> String {
    let filename = format!("../ml47-minirust/{}", filename);
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
