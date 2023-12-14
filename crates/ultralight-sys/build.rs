use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    ultralight_build::build();

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
}
