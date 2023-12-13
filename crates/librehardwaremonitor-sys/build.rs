use std::{path::PathBuf, process::Command};

fn compile_dotnet_project() {
    let output = Command::new("dotnet")
        .args([
            "publish",
            "-r",
            "win-x64",
            "-c",
            "Release",
            "/p:NativeLib=Static",
            "/p:SelfContained=true",
            "/t:LinkNative",
        ])
        .current_dir("./csharp-project")
        .output()
        .expect("Failed to compile LibreHardwareMonitor");
    if !output.status.success() {
        panic!("Failed to compile LibreHardwareMonitor");
    }
}

fn get_lib_path() -> PathBuf {
    let mut manifest_dir: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
    manifest_dir.push("csharp-project");
    manifest_dir.push("bin");
    manifest_dir.push("Release");
    manifest_dir.push("net7.0");
    manifest_dir.push("win-x64");
    manifest_dir.push("publish");
    manifest_dir
}

fn main() {
    compile_dotnet_project();

    println!(
        "cargo:rustc-link-search=native={}",
        get_lib_path().display()
    );
    println!("cargo:rustc-link-lib=static=LibreHardwareMonitorNative");
}
