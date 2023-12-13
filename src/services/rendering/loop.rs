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

    view.load_url("http://127.0.0.1:2137")
        .await
        .expect("Failed to load URL");
    // let fps = (1 / 30) * 1000;
    let mut interval = time::interval(Duration::from_millis(40));

    let mut device = find_and_create_device().await.unwrap();
    device.initialise().await.unwrap();

    // renderer.update();
    // renderer.render();
    loop {
        renderer.update();
        renderer.render();
        let mut surface = view.get_surface();
        let bitmap = surface.get_bitmap();
        if let Ok(mut bitmap) = bitmap {
            if bitmap.swap_red_blue_channels().is_ok() {
                device
                    .send_image(&*bitmap.lock_pixels().unwrap())
                    .await
                    .unwrap();
                bitmap.swap_red_blue_channels();
            }
        }

        interval.tick().await;
    }
}
