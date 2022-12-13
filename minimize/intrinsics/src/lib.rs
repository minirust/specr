use std::fmt::Display;

/// When used by `minimize`, a call to this function will be replaced by a `CallIntrinsic::PrintStdout`.
pub fn print(t: impl Display) {
    println!("{}", t);
}
