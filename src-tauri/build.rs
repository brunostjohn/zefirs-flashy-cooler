fn main() {
    println!("cargo:rustc-link-search=./static-libs/");
    println!("cargo:rustc-link-lib=LibreHardwareMonitorNative");

    println!("cargo:rustc-link-search=/Users/bruno/.nuget/packages/runtime.win-x64.microsoft.dotnet.ilcompiler/7.0.8/sdk");
    println!("cargo:rustc-link-search=/VulkanSDK/1.3.250.1/Lib");
    println!("cargo:rustc-link-arg-bins=/INCLUDE:NativeAOT_StaticInitialization");

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
