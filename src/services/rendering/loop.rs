use tokio::time::{self, Duration};
use ultralight::{ULPlatformBuilder, ULRendererBuilder, ULViewBuilder};

pub async fn main_loop() {
    ULPlatformBuilder::new()
        .enable_file_logger("./logs.txt")
        .enable_platform_file_system()
        .enable_platform_font_loader()
        .build();

    let renderer = ULRendererBuilder::new()
        .set_resource_path_prefix({
            let mut newthing = std::env::current_dir().unwrap();
            newthing.push("target");
            newthing.push("debug");
            newthing.push("deps");
            newthing.push("resources\\");

            newthing
        })
        .build();

    let mut view = ULViewBuilder::new(&renderer)
        .set_width(480)
        .set_height(480)
        .build();

    let loaded = view
        .load_url("https://google.com")
        .await
        .expect("Failed to load URL");
    let fps = 1000 / 30;
    let mut interval = time::interval(Duration::from_millis(fps));

    loop {
        interval.tick().await;
    }
}
