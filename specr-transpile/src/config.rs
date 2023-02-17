use std::fs;

pub struct Config {
    pub attrs: Vec<String>,
}

impl Config {
    pub fn load() -> Config {
        let s = fs::read_to_string("../specr.cfg").unwrap();

        let mut attrs = Vec::new();
        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() { continue; }

            if line.starts_with("attr ") {
                attrs.push(line[4..].to_string());
            }
        }

        Config { attrs }
    }
}
