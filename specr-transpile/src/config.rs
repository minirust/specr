use std::path::{Path, PathBuf};
use std::fs;

pub struct Config {
    /// config root directory.
    pub root: PathBuf,

    /// input path, this is where the original .md files are stored.
    pub input: String,

    /// output path, this is where the crate will be constructed.
    pub output: String,

    /// extra inner attributes for the generated rust crate.
    pub attrs: Vec<String>,
}

impl Config {
    pub fn load() -> Config {
        let [_, ref f] = std::env::args().collect::<Vec<_>>()[..] else {
            eprintln!("Usage:");
            eprintln!("specr-transpile <specr.cfg>");
            eprintln!("");
            panic!("invalid amount of command-line arguments!");
        };
        let f = fs::canonicalize(f).unwrap();
        let s = fs::read_to_string(&f).unwrap();

        let root = f.parent().unwrap().to_path_buf();

        let mut input = None;
        let mut output = None;
        let mut attrs = Vec::new();

        for line in s.lines() {
            let line = line.trim();
            if line.is_empty() { continue; }

            if line.starts_with("input ") {
                assert!(input.is_none());
                input = Some(line[5..].trim().to_string());
            }

            if line.starts_with("output ") {
                assert!(output.is_none());
                output = Some(line[6..].trim().to_string());
            }

            if line.starts_with("attr ") {
                attrs.push(line[4..].to_string());
            }
        }

        Config {
            root,
            input: input.unwrap(),
            output: output.unwrap(),
            attrs
        }
    }

    pub fn input_path(&self) -> PathBuf {
        self.canonicalize(&self.input)
    }

    pub fn output_path(&self) -> PathBuf {
        self.canonicalize(&self.output)
    }

    pub fn crate_name(&self) -> String {
        self.output_path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string()
    }

    // converts relative paths to be relative from `root`
    fn canonicalize(&self, t: impl AsRef<Path>) -> PathBuf {
        let path = t.as_ref();
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.root.join(path)
        }
    }
}
