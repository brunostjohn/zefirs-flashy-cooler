use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let header_location = {
        let mut dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR")
            .expect("Failed to get project dir!")
            .into();
        dir.push("wrapped");
        dir.push("ultralight");

        dir
    };

    ultralight_build::build_with_headers(header_location);

    let bindings = bindgen::Builder::default()
        .header("wrapped/wrapper.h")
        .clang_arg("-I./wrapped/ultralight")
        .impl_debug(true)
        .impl_partialeq(true)
        .generate_comments(true)
        .generate_inline_functions(true)
        .allowlist_var("^UL.*|JS.*|ul.*|WK.*|kJS.*")
        .allowlist_type("^UL.*|JS.*|ul.*|WK.*")
        .allowlist_function("^UL.*|JS.*|ul.*|WK.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
