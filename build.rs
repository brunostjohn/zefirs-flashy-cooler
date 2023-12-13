use tauri_build::{Attributes, WindowsAttributes};

fn main() -> anyhow::Result<()> {
    ultralight_build::build();

    tauri_build::try_build(Attributes::default().windows_attributes(
        WindowsAttributes::default().app_manifest(include_str!("./manifest.xml")),
    ))
}
