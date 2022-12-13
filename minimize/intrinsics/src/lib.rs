use std::fmt::Display;

/// When used by `minimize`, a call to this function will be replaced by a `CallIntrinsic::PrintStdout`.
/// The body of this function is mostly used through `tests/rust.sh`.
pub fn print(t: impl Display) {
    println!("{}", t);
}
