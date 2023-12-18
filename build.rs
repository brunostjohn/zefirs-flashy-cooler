use tauri_build::{Attributes, WindowsAttributes};

fn main() -> anyhow::Result<()> {
    dotnetaot_build::use_aot_lib();
    ultralight_build::build();

    tauri_build::try_build(Attributes::default().windows_attributes(
        WindowsAttributes::default().app_manifest(include_str!("./manifest.xml")),
    ))
}
