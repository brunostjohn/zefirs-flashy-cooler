fn main() {
    println!("cargo:rustc-link-search=./static-libs/");
    tauri_build::build()
}
