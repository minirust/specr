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

    /// The rust channel, like "nightly"
    /// If this is `Some`, a `rust-toolchain.toml` will be created in the generated crate.
    pub channel: Option<String>,

    /// The name of the generated crate.
    pub name: String,
}

impl Config {
    pub fn load() -> Config {
        let [_, ref f] = std::env::args().collect::<Vec<_>>()[..] else {
            eprintln!("Usage:");
            eprintln!("specr-transpile <specr.toml>");
            eprintln!("");
            panic!("invalid amount of command-line arguments!");
        };

        let f = fs::canonicalize(f).unwrap();
        let s = fs::read_to_string(&f).unwrap();
        let root = f.parent().unwrap().to_path_buf();

        let table = s.parse::<toml::Table>().unwrap();
        let input = table.get("input").expect("`input` missing in config file")
                         .as_str().expect("`input` is no string!").to_string();
        let output = table.get("output").expect("`output` missing in config file")
                         .as_str().expect("`output` is no string!").to_string();
        let attrs = table.get("attrs")
                          .map(|v| v.clone().try_into().expect("`attrs` is required to be an array of strings!"))
                          .unwrap_or_else(Vec::new);
        let channel = table.get("channel")
                          .map(|v| v.clone().try_into().expect("`channel` is required to be a string!"));
        let name = table.get("name").expect("`name` is missing in config file")
                          .clone().try_into().expect("`name` is required to be a string!");

        Config {
            root,
            input,
            output,
            attrs,
            channel,
            name,
        }
    }

    pub fn input_path(&self) -> PathBuf {
        self.canonicalize(&self.input)
    }

    pub fn output_path(&self) -> PathBuf {
        self.canonicalize(&self.output)
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
