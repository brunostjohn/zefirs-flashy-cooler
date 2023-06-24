fn main() {
    println!("cargo:rustc-link-search=./static-libs/");
    let mut windows = tauri_build::WindowsAttributes::default();
    windows = windows.app_manifest(
        r#"<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
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
