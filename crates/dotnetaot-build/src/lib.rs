use std::{path::PathBuf, process::Command};

fn compile_dotnet_project(csharp_project_path: Option<PathBuf>) {
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
        .current_dir(
            csharp_project_path
                .and_then(|val| val.to_str().map(|val| val.to_owned()))
                .unwrap_or("./csharp-project".into()),
        )
        .output()
        .expect("Failed to compile LibreHardwareMonitor");
    if !output.status.success() {
        panic!("Failed to compile LibreHardwareMonitor");
    }
}

fn get_lib_path(csharp_project_path: Option<PathBuf>) -> PathBuf {
    let mut manifest_dir: PathBuf = csharp_project_path.unwrap_or({
        let mut manifest: PathBuf = std::env::var("CARGO_MANIFEST_DIR").unwrap().into();
        manifest.push("csharp-project");
        manifest
    });
    manifest_dir.push("bin");
    manifest_dir.push("Release");
    manifest_dir.push("net7.0");
    manifest_dir.push("win-x64");
    manifest_dir.push("publish");
    manifest_dir
}

fn get_aot_libs_path() -> PathBuf {
    let user_path = std::env::var("USERPROFILE").unwrap();
    let mut aot_libs_path: PathBuf = user_path.into();
    aot_libs_path.push(".nuget");
    aot_libs_path.push("packages");
    aot_libs_path.push("runtime.win-x64.microsoft.dotnet.ilcompiler");
    aot_libs_path.push("7.0.10");
    aot_libs_path.push("sdk");

    aot_libs_path
}

pub fn use_aot_lib() {
    let aot_libs_path = get_aot_libs_path();
    println!("cargo:rustc-link-search=native={}", aot_libs_path.display());
    println!("cargo:rustc-link-arg=/INCLUDE:NativeAOT_StaticInitialization");

    println!("cargo:rustc-link-lib=Ole32");
    println!("cargo:rustc-link-lib=OleAut32");
    println!("cargo:rustc-link-lib=Iphlpapi");
    println!("cargo:rustc-link-lib=Crypt32");
}

pub fn build(lib_name: &str, csharp_project_path: Option<PathBuf>) {
    compile_dotnet_project(csharp_project_path.clone());
    let aot_libs_path = get_aot_libs_path();

    println!("cargo:rustc-link-search=native={}", aot_libs_path.display());
    println!("cargo:rustc-link-arg=/INCLUDE:NativeAOT_StaticInitialization");
    println!(
        "cargo:rustc-link-search=native={}",
        get_lib_path(csharp_project_path).display()
    );

    println!("cargo:rustc-link-lib=Ole32");
    println!("cargo:rustc-link-lib=OleAut32");
    println!("cargo:rustc-link-lib=Iphlpapi");
    println!("cargo:rustc-link-lib=Crypt32");

    println!("cargo:rustc-link-lib=static={}", lib_name);
}
