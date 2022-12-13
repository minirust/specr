extern crate ui_test;
use std::process::Command;

fn main() {
    // first, compile the `intrinsics` crate.
    Command::new("cargo")
             .arg("build")
             .current_dir("./intrinsics")
             .output()
             .expect("Failed to compile `intrinsics`!");

    let mut cfg = ui_test::Config::default();
    cfg.args.clear();
    cfg.program = std::path::PathBuf::from("./target/debug/minimize");
    cfg.root_dir = std::path::PathBuf::from("./tests/files");
    cfg.host = Some(String::new());
    cfg.mode = ui_test::Mode::Pass;
    ui_test::run_tests(cfg).unwrap();
}
