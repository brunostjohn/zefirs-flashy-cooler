use crate::utils::app::paths::get_app_path;
use anyhow::Context;
use lcd_coolers::{find_and_create_device, DeviceCreator, DisplayCooler};
use ultralight::{ULPlatformBuilder, ULRenderer, ULRendererBuilder};

pub async fn setup_rendering() -> anyhow::Result<(ULRenderer, impl DisplayCooler + DeviceCreator)> {
    let app_path = get_app_path()?;

    ULPlatformBuilder::new()
        .enable_file_logger(app_path.join("ul-log.txt"))
        .enable_platform_file_system()
        .enable_platform_font_loader()
        .build();

    let renderer = ULRendererBuilder::new()
        .set_resource_path_prefix(app_path.join("resources"))
        .build();

    let mut device = find_and_create_device().await.context("No device found!")?;
    device
        .initialise()
        .await
        .context("Failed to initialise device!")?;

    Ok((renderer, device))
}
