use std::fs;

use std::io;
use std::path::{Path, PathBuf};

// recursively copies directory.
pub fn cp_dir(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    if dst.exists() {
        fs::remove_dir_all(dst)?;
    }
    fs::create_dir(dst)?;

    for f in fs::read_dir(src)? {
        let f = f?;
        let filename = f.file_name();

        let src: PathBuf = [src, filename.as_ref()].iter().collect();
        let dst: PathBuf = [dst, filename.as_ref()].iter().collect();

        let ftype = f.file_type()?;
        if ftype.is_dir() {
            cp_dir(src, dst)?;
        } else if ftype.is_file() {
            fs::copy(src, dst)?;
        } else {
            panic!("unexpected file type in cp_dir!");
        }
    }

    Ok(())
}
