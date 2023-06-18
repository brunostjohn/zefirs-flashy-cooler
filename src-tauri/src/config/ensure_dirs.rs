use crate::THEMES_PATH;
use std::fs;

pub fn ensure_dirs() {
    let f_path = THEMES_PATH.as_path();

    if !f_path.exists() {
        fs::create_dir(f_path).unwrap();
    }
}
