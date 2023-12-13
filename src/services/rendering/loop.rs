use lcd_coolers::{find_and_create_device, DisplayCooler};
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

    let mut device = find_and_create_device().await.unwrap();
    device.initialise().await.unwrap();

    loop {
        renderer.update();
        renderer.render();
        let mut surface = view.get_surface();
        let bitmap = surface.get_bitmap();
        if let Ok(mut bitmap) = bitmap {
            if bitmap.swap_red_blue_channels().is_ok() {
                let pixels = bitmap.lock_pixels();

                if let Some(pixels) = pixels {
                    device.send_image(&*pixels).await.unwrap();
                }
            }
        }

        interval.tick().await;
    }
}
