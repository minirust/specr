//! When used by `minimize`, a call to these functions will be replaced by a `CallIntrinsic`.
//! The bodies of these functions are mostly used through `tests/rust.sh`.

use std::fmt::Display;

pub fn print(t: impl Display) {
    println!("{}", t);
}

pub fn eprint(t: impl Display) {
    eprintln!("{}", t);
}

pub fn exit() {
    std::process::exit(0);
}
