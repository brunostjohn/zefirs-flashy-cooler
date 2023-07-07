use std::{fs, path::PathBuf};

pub fn ensure_dirs(themes_path: PathBuf) {
    let f_path = themes_path.as_path();

    if !f_path.exists() {
        fs::create_dir(f_path).unwrap();
    }
}
