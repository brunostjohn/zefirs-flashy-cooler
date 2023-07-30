use std::{io::Cursor, path::Path};

fn main() {
    ensure_static_lib_dir();
    ensure_resources_dir();

    if let Err(DepError::NotFound) = check_ultralight() {
        download_ultralight().unwrap();
    }

    if let Err(DepError::NotFound) = check_aot() {
        download_aot().unwrap();
    }

    println!("cargo:rustc-link-search=./static-libs/");
    println!("cargo:rustc-link-lib=LibreHardwareMonitorNative");
    println!("cargo:rustc-link-arg-bins=/INCLUDE:NativeAOT_StaticInitialization");

    rerun_if_changed(&[
        "./build.rs",
        "./static-libs/LibreHardwareMonitorNative.lib",
        // Ultralight
        "./static-libs/AppCore.lib",
        "./static-libs/Ultralight.lib",
        "./static-libs/UltralightCore.lib",
        "./static-libs/WebCore.lib",
        // .NET AOT
        "./static-libs/bootstrapperdll.lib",
        "./static-libs/System.IO.Compression.Native.Aot.lib",
        "./static-libs/Runtime.WorkstationGC.lib",
        "./static-libs/System.Globalization.Native.Aot.lib",
        // Manifest
        "./app_manifest.xml",
    ]);

    let mut windows = tauri_build::WindowsAttributes::default();
    windows = windows.app_manifest(include_str!("./app_manifest.xml"));
    let attrs = tauri_build::Attributes::default().windows_attributes(windows);

    tauri_build::try_build(attrs).unwrap()
}

fn ensure_static_lib_dir() {
    let lib_dir = {
        let mut temp = std::env::current_dir().unwrap();
        temp.push("static-libs");

        temp
    };

    if !lib_dir.exists() {
        std::fs::create_dir(lib_dir).unwrap();
    }
}

fn ensure_resources_dir() {
    let lib_dir = {
        let mut temp = std::env::current_dir().unwrap();
        temp.push("resources");

        temp
    };

    if !lib_dir.exists() {
        std::fs::create_dir(lib_dir).unwrap();
    }
}

#[derive(Debug)]
enum DepError {
    OpFailed,
    NotFound,
}

fn check_aot() -> Result<(), DepError> {
    let aot_deps = [
        "bootstrapperdll.lib",
        "System.IO.Compression.Native.Aot.lib",
        "Runtime.WorkstationGC.lib",
        "System.Globalization.Native.Aot.lib",
    ];

    let cwd = {
        let mut temp = std::env::current_dir().or(Err(DepError::OpFailed))?;
        temp.push("static-libs");
        temp
    };

    for dep in aot_deps {
        let dep_path = {
            let mut temp = cwd.clone();

            temp.push(dep);

            temp
        };

        if !dep_path.exists() {
            return Err(DepError::NotFound);
        }
    }

    Ok(())
}

fn download_aot() -> Result<(), DepError> {
    let aot_url =
        "https://www.nuget.org/api/v2/package/runtime.win-x64.Microsoft.DotNet.ILCompiler/7.0.9";

    let response = reqwest::blocking::get(aot_url).or(Err(DepError::OpFailed))?;

    let content = Cursor::new(response.bytes().or(Err(DepError::OpFailed))?);

    let target = {
        let mut temp = std::env::temp_dir();

        temp.push("ilcompiler");

        temp
    };

    if target.exists() {
        std::fs::remove_dir_all(&target).or(Err(DepError::OpFailed))?;
    }

    std::fs::create_dir(&target).or(Err(DepError::OpFailed))?;

    zip_extract::extract(content, &target, false).or(Err(DepError::OpFailed))?;

    let cwd = std::env::current_dir().or(Err(DepError::OpFailed))?;

    let aot_deps = [
        "bootstrapperdll.lib",
        "System.IO.Compression.Native.Aot.lib",
        "Runtime.WorkstationGC.lib",
        "System.Globalization.Native.Aot.lib",
    ];

    for dep in aot_deps {
        let from = {
            let mut temp = target.clone();
            temp.push("sdk");
            temp.push(dep);

            temp
        };

        let to = {
            let mut temp = cwd.clone();
            temp.push("static-libs");
            temp.push(dep);

            temp
        };

        std::fs::copy(from, to).or(Err(DepError::OpFailed))?;
    }

    fix_compression_lib();

    std::fs::remove_dir_all(&target).or(Err(DepError::OpFailed))?;

    Ok(())
}

fn fix_compression_lib() {
    let target = std::env::var("TARGET").unwrap();
    let lib_tool = cc::windows_registry::find_tool(&target, "lib.exe")
                .expect("Could not find \"lib.exe\". Please ensure a supported version of Visual Studio is installed.");

    let mut lib_cmd = lib_tool.to_command();

    let lib_path = {
        let mut temp = std::env::current_dir().unwrap();
        temp.push("static-libs");
        temp.push("System.IO.Compression.Native.Aot.lib");

        temp
    };

    let lib_path_str = lib_path.to_string_lossy();

    lib_cmd.arg("/remove:libs-native\\System.IO.Compression.Native\\CMakeFiles\\System.IO.Compression.Native.Aot.dir\\D_\\a\\_work\\1\\s\\src\\native\\external\\brotli\\dec\\decode.c.obj")
    .arg(format!("/LIBPATH:{lib_path_str}"))
    .arg(format!("{lib_path_str}"));

    lib_cmd.output().expect("Failed to run lib.exe.");
}

fn check_ultralight() -> Result<(), DepError> {
    let necessary_dlls = [
        "AppCore.dll",
        "Ultralight.dll",
        "UltralightCore.dll",
        "WebCore.dll",
    ];
    let necessary_libs = [
        "AppCore.lib",
        "Ultralight.lib",
        "UltralightCore.lib",
        "WebCore.lib",
    ];

    let resources = ["cacert.pem", "icudt67l.dat", "inspector"];

    let cwd = std::env::current_dir().or(Err(DepError::OpFailed))?;

    for dll_str in necessary_dlls {
        let dir = {
            let mut temp = cwd.clone();
            temp.push(dll_str);
            temp
        };

        if !dir.exists() {
            return Err(DepError::NotFound);
        }
    }

    for lib_str in necessary_libs {
        let dir = {
            let mut temp = cwd.clone();
            temp.push("static-libs");
            temp.push(lib_str);
            temp
        };

        if !dir.exists() {
            return Err(DepError::NotFound);
        }
    }

    for resource in resources {
        let dir = {
            let mut temp = cwd.clone();
            temp.push("resources");
            temp.push(resource);
            temp
        };

        if !dir.exists() {
            return Err(DepError::NotFound);
        }
    }

    Ok(())
}

fn download_ultralight() -> Result<(), DepError> {
    let ultralight_url =
        "https://ultralight-sdk.sfo2.cdn.digitaloceanspaces.com/ultralight-sdk-latest-win-x64.7z";

    let response = reqwest::blocking::get(ultralight_url).or(Err(DepError::OpFailed))?;

    let content = Cursor::new(response.bytes().or(Err(DepError::OpFailed))?);

    let target = {
        let mut temp = std::env::temp_dir();

        temp.push("ul_sdk");

        temp
    };

    if target.exists() {
        std::fs::remove_dir_all(&target).or(Err(DepError::OpFailed))?;
    }

    std::fs::create_dir(&target).or(Err(DepError::OpFailed))?;

    sevenz_rust::decompress(content, &target).or(Err(DepError::OpFailed))?;

    let necessary_dlls = [
        "AppCore.dll",
        "Ultralight.dll",
        "UltralightCore.dll",
        "WebCore.dll",
    ];
    let necessary_libs = [
        "AppCore.lib",
        "Ultralight.lib",
        "UltralightCore.lib",
        "WebCore.lib",
    ];

    let cwd = std::env::current_dir().or(Err(DepError::OpFailed))?;

    for dll_str in necessary_dlls {
        let dir = {
            let mut temp = cwd.clone();
            temp.push(dll_str);
            temp
        };

        let from = {
            let mut temp = target.clone();
            temp.push("bin");
            temp.push(dll_str);
            temp
        };

        std::fs::copy(from, dir).or(Err(DepError::OpFailed))?;
    }

    for lib_str in necessary_libs {
        let dir = {
            let mut temp = cwd.clone();
            temp.push("static-libs");
            temp.push(lib_str);
            temp
        };

        let from = {
            let mut temp = target.clone();
            temp.push("lib");
            temp.push(lib_str);
            temp
        };

        std::fs::copy(from, dir).or(Err(DepError::OpFailed))?;
    }

    let resources = ["cacert.pem", "icudt67l.dat"];

    for resource in resources {
        let dir = {
            let mut temp = cwd.clone();
            temp.push("resources");
            temp.push(resource);
            temp
        };

        let from = {
            let mut temp = target.clone();
            temp.push("resources");
            temp.push(resource);
            temp
        };

        std::fs::copy(from, dir).or(Err(DepError::OpFailed))?;
    }

    let inspector = {
        let mut temp = target.clone();
        temp.push("inspector");
        temp
    };

    let inspector_to = {
        let mut temp = cwd.clone();
        temp.push("resources");
        temp.push("inspector");

        temp
    };

    copy_dir_all(inspector, inspector_to).unwrap();

    std::fs::remove_dir_all(&target).or(Err(DepError::OpFailed))?;

    Ok(())
}

fn rerun_if_changed(files: &[&str]) {
    for file in files {
        println!("cargo:rerun-if-changed={file}");
    }
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    std::fs::create_dir_all(&dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            std::fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
