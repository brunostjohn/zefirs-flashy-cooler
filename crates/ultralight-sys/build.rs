use std::fs;
use std::path::{Path, PathBuf};
use std::{env, io};

const WINDOWS_DL: &str =
    "https://ultralight-sdk.sfo2.cdn.digitaloceanspaces.com/ultralight-sdk-latest-win-x64.7z";

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn main() {
    let bundle = reqwest::blocking::get(WINDOWS_DL)
        .expect("Failed to download Ultralight bundle!")
        .bytes()
        .expect("Failed to get Ultralight bytes!")
        .to_vec();
    let cursor = std::io::Cursor::new(bundle);
    let dir = {
        let mut dir = std::env::temp_dir();
        dir.push("ultralight");

        if dir.exists() {
            let _ = std::fs::remove_dir_all(&dir);
        }
        std::fs::create_dir_all(&dir).expect("Failed to create Ultralight temp dir!");

        dir
    };
    sevenz_rust::decompress(cursor, &dir).expect("Failed to decompress Ultralight bundle!");

    let include_dir = dir.join("include");
    let dst =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("Failed to find cargo manifest!"))
            .join("wrapped")
            .join("ultralight");
    if dst.exists() {
        std::fs::remove_dir_all(&dst).expect("Failed to remove old Ultralight includes!");
    }
    copy_dir_all(include_dir, dst).expect("Failed to copy Ultralight includes!");

    let lib_dir = dir.join("lib");
    let target_dir: PathBuf = std::env::var("OUT_DIR")
        .expect("Failed to get target dir!")
        .into();
    let target_dir = target_dir.join("ul-libs");
    if target_dir.exists() {
        let _ = std::fs::remove_dir_all(&target_dir);
    }
    std::fs::create_dir(&target_dir).expect("Failed to create ul-libs dir!");
    copy_dir_all(lib_dir, &target_dir).expect("Failed to copy ultralight libs!");

    println!("cargo:rustc-link-lib=Ultralight");
    println!("cargo:rustc-link-lib=WebCore");
    println!("cargo:rustc-link-lib=AppCore");

    println!("cargo:rustc-link-search=native={}", target_dir.display());

    let bindings = bindgen::Builder::default()
        .header("wrapped/wrapper.h")
        .clang_arg("-I./wrapped/ultralight")
        .impl_debug(true)
        .impl_partialeq(true)
        .generate_comments(true)
        .generate_inline_functions(true)
        .allowlist_var("^UL.*|JS.*|ul.*|WK.*")
        .allowlist_type("^UL.*|JS.*|ul.*|WK.*")
        .allowlist_function("^UL.*|JS.*|ul.*|WK.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    let bin_dir = dir.join("bin");
    let target_dir: PathBuf = std::env::var("OUT_DIR")
        .expect("Failed to get target dir!")
        .into();
    let target_dir = target_dir.join("ul-bin");
    if target_dir.exists() {
        let _ = std::fs::remove_dir_all(&target_dir);
    }
    std::fs::create_dir(&target_dir).expect("Failed to create ul-bin dir!");
    copy_dir_all(bin_dir, &target_dir).expect("Failed to copy ultralight bins!");

    let resources_dir = dir.join("resources");
    let target_dir: PathBuf = std::env::var("OUT_DIR")
        .expect("Failed to get target dir!")
        .into();
    let target_dir = target_dir.join("ul-resources");
    if target_dir.exists() {
        let _ = std::fs::remove_dir_all(&target_dir);
    }
    std::fs::create_dir(&target_dir).expect("Failed to create ul-resources dir!");
    copy_dir_all(resources_dir, &target_dir).expect("Failed to copy ultralight resources!");
}
