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

fn create_and_fix_brotli(path: PathBuf) {
    let user_path = std::env::var("USERPROFILE").unwrap();
    let mut aot_libs_path: PathBuf = user_path.into();
    aot_libs_path.push(".nuget");
    aot_libs_path.push("packages");
    aot_libs_path.push("runtime.win-x64.microsoft.dotnet.ilcompiler");
    aot_libs_path.push("7.0.10");
    aot_libs_path.push("sdk");

    let aot_deps = [
        "bootstrapperdll.lib",
        "System.IO.Compression.Native.Aot.lib",
        "Runtime.WorkstationGC.lib",
        "System.Globalization.Native.Aot.lib",
    ];

    for dep in aot_deps {
        let mut dep_path = aot_libs_path.clone();
        dep_path.push(dep);
        let mut dep_path_out = path.clone();
        dep_path_out.push(dep);
        std::fs::copy(dep_path, dep_path_out).unwrap();
    }

    let target = std::env::var("TARGET").unwrap();
    let lib_tool = cc::windows_registry::find_tool(&target, "lib.exe")
    .expect("Could not find \"lib.exe\". Please ensure a supported version of Visual Studio is installed.");

    let mut lib_cmd = lib_tool.to_command();
    let out_dir: PathBuf = std::env::var("OUT_DIR").unwrap().into();
    let aot_libs_path: PathBuf = out_dir.join("aot-libs");

    let lib_path = {
        let mut temp = aot_libs_path.clone();
        temp.push("System.IO.Compression.Native.Aot.lib");

        temp
    };

    let lib_path_str = lib_path.to_string_lossy();

    lib_cmd.arg("/remove:libs-native\\System.IO.Compression.Native\\CMakeFiles\\System.IO.Compression.Native.Aot.dir\\D_\\a\\_work\\1\\s\\src\\native\\external\\brotli\\dec\\decode.c.obj")
    .arg(format!("/LIBPATH:{lib_path_str}"))
    .arg(format!("{lib_path_str}"));

    let output = lib_cmd.output().expect("Failed to run lib.exe.");
    if !output.status.success() {
        panic!(
            "Failed to run lib.exe.\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn get_aot_libs_path() -> PathBuf {
    let remove_brotli = std::env::var("AOT_COMPRESSION_REMOVE_BROTLI").unwrap() == "TRUE";
    if !remove_brotli {
        let user_path = std::env::var("USERPROFILE").unwrap();
        let mut aot_libs_path: PathBuf = user_path.into();
        aot_libs_path.push(".nuget");
        aot_libs_path.push("packages");
        aot_libs_path.push("runtime.win-x64.microsoft.dotnet.ilcompiler");
        aot_libs_path.push("7.0.10");
        aot_libs_path.push("sdk");

        aot_libs_path
    } else {
        let out_dir: PathBuf = std::env::var("OUT_DIR").unwrap().into();
        let aot_libs_path: PathBuf = out_dir.join("aot-libs");
        if !aot_libs_path.exists() {
            std::fs::create_dir_all(&aot_libs_path).unwrap();
        }

        create_and_fix_brotli(aot_libs_path.clone());

        aot_libs_path
    }
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
