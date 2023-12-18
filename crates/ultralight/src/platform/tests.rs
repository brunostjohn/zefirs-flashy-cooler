use super::*;

#[test]
fn configures_platform() {
    ULPlatformBuilder::new()
        .enable_file_logger("./logs.txt")
        .enable_platform_file_system()
        .enable_platform_font_loader()
        .build();
}
