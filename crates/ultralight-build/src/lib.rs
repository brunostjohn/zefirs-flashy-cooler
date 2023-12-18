use std::{
    fs,
    io::{self, Cursor},
    path::{Path, PathBuf},
};

const DOWNLOAD_URL: &str = "https://ultralight-sdk.sfo2.cdn.digitaloceanspaces.com/ultralight-sdk-latest-win-x64.7z";

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

fn get_headers(header_location: PathBuf) {
    let dir = {
        if std::env::var("DOCS_RS").is_ok() {
            let mut dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR")
                .expect("Failed to get project dir!")
                .into();
            dir.push("ultralight-deps");
            dir.push("include");

            std::fs::create_dir_all(&dir).expect("Failed to create Ultralight deps dir!");

            dir
        } else {
            let mut dir: PathBuf = std::env::temp_dir();
            dir.push("ultralight-deps");
            std::fs::create_dir_all(&dir).expect("Failed to create Ultralight deps dir!");

            let bytes = reqwest::blocking::get(DOWNLOAD_URL)
                .expect("Failed to download Ultralight resources!")
                .bytes()
                .expect("Failed to get Ultralight bytes!")
                .to_vec();

            sevenz_rust::decompress(Cursor::new(bytes), &dir)
                .expect("Failed to decompress UL resources!");

            dir.push("include");

            dir
        }
    };

    fs::create_dir_all(&header_location).expect("Failed to create header dir!");
    copy_dir_all(dir, header_location).expect("Failed to copy headers!");
}

fn download_resources() {
    let dir = {
        if std::env::var("DOCS_RS").is_ok() {
            let mut dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR")
                .expect("Failed to get project dir!")
                .into();
            dir.push("ultralight-deps");

            std::fs::create_dir_all(&dir).expect("Failed to create Ultralight deps dir!");

            dir
        } else {
            let mut dir: PathBuf = std::env::temp_dir();
            dir.push("ultralight-deps");
            std::fs::create_dir_all(&dir).expect("Failed to create Ultralight deps dir!");

            let bytes = reqwest::blocking::get(DOWNLOAD_URL)
                .expect("Failed to download Ultralight resources!")
                .bytes()
                .expect("Failed to get Ultralight bytes!")
                .to_vec();

            sevenz_rust::decompress(Cursor::new(bytes), &dir)
                .expect("Failed to decompress UL resources!");

            dir
        }
    };

    let lib_dir = dir.join("lib");
    let target_dir: PathBuf = std::env::var("OUT_DIR")
        .expect("Failed to get target dir!")
        .into();
    copy_dir_all(lib_dir, target_dir).expect("Failed to copy ultralight libs!");

    if std::env::var("DOCS_RS").is_err() {
        let bin_dir = dir.join("bin");
        let mut target_dir: PathBuf = std::env::var("OUT_DIR")
            .expect("Failed to get target dir!")
            .into();
        target_dir.pop();
        target_dir.pop();
        target_dir.pop();
        copy_dir_all(bin_dir, target_dir).expect("Failed to copy ultralight bins!");

        let resources_dir = dir.join("resources");
        let mut target_dir: PathBuf = std::env::var("OUT_DIR")
            .expect("Failed to get target dir!")
            .into();
        target_dir.pop();
        target_dir.pop();
        target_dir.pop();
        let target_dir = target_dir.join("resources");
        if target_dir.exists() {
            std::fs::remove_dir_all(&target_dir)
                .expect("Failed to remove old Ultralight resources!");
        } else {
            std::fs::create_dir_all(&target_dir)
                .expect("Failed to create Ultralight resources dir!");
        }
        copy_dir_all(resources_dir, target_dir).expect("Failed to copy Ultralight resources!");
    }
}

fn validate() -> bool {
    let target_dir: PathBuf = std::env::var("OUT_DIR")
        .expect("Failed to get target dir!")
        .into();
    let lib = target_dir.join("Ultralight.lib");
    if !lib.exists() {
        return false;
    }
    let lib = target_dir.join("UltralightCore.lib");
    if !lib.exists() {
        return false;
    }
    let lib = target_dir.join("AppCore.lib");
    if !lib.exists() {
        return false;
    }
    let lib = target_dir.join("WebCore.lib");
    if !lib.exists() {
        return false;
    }
    let mut target_dir: PathBuf = std::env::var("OUT_DIR")
        .expect("Failed to get target dir!")
        .into();
    target_dir.pop();
    target_dir.pop();
    target_dir.pop();
    let lib = target_dir.join("Ultralight.dll");
    if !lib.exists() {
        return false;
    }
    let lib = target_dir.join("UltralightCore.dll");
    if !lib.exists() {
        return false;
    }
    let lib = target_dir.join("AppCore.dll");
    if !lib.exists() {
        return false;
    }
    let lib = target_dir.join("WebCore.dll");
    if !lib.exists() {
        return false;
    }
    let resource = target_dir.join("resources");
    if !resource.exists() {
        return false;
    }

    true
}

pub fn build() {
    if !validate() {
        download_resources();
    }

    let target_dir: PathBuf = std::env::var("OUT_DIR")
        .expect("Failed to get target dir!")
        .into();

    println!("cargo:rustc-link-search=native={}", target_dir.display());
    println!("cargo:rustc-link-lib=Ultralight");
    println!("cargo:rustc-link-lib=WebCore");
    println!("cargo:rustc-link-lib=AppCore");
}

pub fn build_with_headers(header_location: PathBuf) {
    if !validate() {
        download_resources();
        get_headers(header_location);
    }

    let target_dir: PathBuf = std::env::var("OUT_DIR")
        .expect("Failed to get target dir!")
        .into();

    println!("cargo:rustc-link-search=native={}", target_dir.display());
    println!("cargo:rustc-link-lib=Ultralight");
    println!("cargo:rustc-link-lib=WebCore");
    println!("cargo:rustc-link-lib=AppCore");
}
