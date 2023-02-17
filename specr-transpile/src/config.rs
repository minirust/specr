use std::fs;

pub struct Config {
    pub input: String,
    pub output: String,
    pub attrs: Vec<String>,
}

impl Config {
    pub fn load() -> Config {
        let s = fs::read_to_string("../specr.cfg").unwrap();

        let mut input = None;
        let mut output = None;
        let mut attrs = Vec::new();

        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() { continue; }

            if line.starts_with("input ") {
                input = Some(line[5..].trim().to_string());
            }

            if line.starts_with("output ") {
                output = Some(line[6..].trim().to_string());
            }

            if line.starts_with("attr ") {
                attrs.push(line[4..].to_string());
            }
        }

        Config {
            input: input.unwrap(),
            output: output.unwrap(),
            attrs
        }
    }
}
