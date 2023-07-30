use std::io::Cursor;

#[derive(Debug)]
enum ULError {
    OpFailed,
    NotFound,
}

fn check_ultralight() -> Result<(), ULError> {
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

    let cwd = std::env::current_dir().or(Err(ULError::OpFailed))?;

    for dll_str in necessary_dlls {
        let dir = {
            let mut temp = cwd.clone();
            temp.push(dll_str);
            temp
        };

        if !dir.exists() {
            return Err(ULError::NotFound);
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
            return Err(ULError::NotFound);
        }
    }

    Ok(())
}

fn download_ultralight() -> Result<(), ULError> {
    let ultralight_url =
        "https://ultralight-sdk.sfo2.cdn.digitaloceanspaces.com/ultralight-sdk-latest-win-x64.7z";

    let response = reqwest::blocking::get(ultralight_url).or(Err(ULError::OpFailed))?;

    let archive_path = {
        let mut temp = std::env::temp_dir();

        temp.push("ul_sdk.7z");

        temp
    };

    let mut file = std::fs::File::create(archive_path).or(Err(ULError::OpFailed))?;
    let mut content = Cursor::new(response.bytes().or(Err(ULError::OpFailed))?);

    std::io::copy(&mut content, &mut file).or(Err(ULError::OpFailed))?;

    let target = {
        let mut temp = std::env::temp_dir();

        temp.push("ul_sdk");

        temp
    };

    std::fs::create_dir(&target).or(Err(ULError::OpFailed))?;

    sevenz_rust::decompress(file, &target).or(Err(ULError::OpFailed))?;

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

    let cwd = std::env::current_dir().or(Err(ULError::OpFailed))?;

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

        std::fs::copy(from, dir).or(Err(ULError::OpFailed))?;
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

        std::fs::copy(from, dir).or(Err(ULError::OpFailed))?;
    }

    Ok(())
}

fn main() {
    if let Err(ULError::NotFound) = check_ultralight() {
        download_ultralight().unwrap();
    }

    println!("cargo:rustc-link-search=./static-libs/");
    println!("cargo:rustc-link-lib=LibreHardwareMonitorNative");

    let home = dirs::home_dir();

    if let Some(dir) = home {
        println!("cargo:rustc-link-search={}/.nuget/packages/runtime.win-x64.microsoft.dotnet.ilcompiler/7.0.8/sdk", dir.display());
    } else {
        println!("cargo:rustc-link-search=/Users/bruno/.nuget/packages/runtime.win-x64.microsoft.dotnet.ilcompiler/7.0.8/sdk");
    }

    println!("cargo:rustc-link-arg-bins=/INCLUDE:NativeAOT_StaticInitialization");

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=./static-libs/AppCore.lib");
    println!("cargo:rerun-if-changed=./static-libs/LibreHardwareMonitorNative.lib");
    println!("cargo:rerun-if-changed=./static-libs/shaderc_shared.lib");
    println!("cargo:rerun-if-changed=./static-libs/Ultralight.lib");
    println!("cargo:rerun-if-changed=./static-libs/UltralightCore.lib");
    println!("cargo:rerun-if-changed=./static-libs/WebCore.lib");

    let mut windows = tauri_build::WindowsAttributes::default();
    windows = windows.app_manifest(
        r#"
            <assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
                <dependency>
                    <dependentAssembly>
                        <assemblyIdentity
                            type="win32"
                            name="Microsoft.Windows.Common-Controls"
                            version="6.0.0.0"
                            processorArchitecture="*"
                            publicKeyToken="6595b64144ccf1df"
                            language="*"
                        />
                    </dependentAssembly>
                </dependency>
                <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
                    <security>
                        <requestedPrivileges>
                            <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
                        </requestedPrivileges>
                    </security>
                </trustInfo>
            </assembly>
        "#,
    );
    let attrs = tauri_build::Attributes::default().windows_attributes(windows);

    tauri_build::try_build(attrs).unwrap()
}
