use super::{
    render_helpers::{render_bitmap_and_send, update_and_render},
    setup::setup_rendering,
};
use lcd_coolers::{DeviceCreator, DeviceInfo};
use tokio::time::{self, Duration};
use ultralight::ULViewBuilder;

pub async fn main_loop() {
    let (renderer, mut device) = setup_rendering().await.expect("Failed to setup rendering!");
    let DeviceInfo { width, height, .. } = device.device_info().await;
    let mut view = ULViewBuilder::new(&renderer)
        .set_width(width)
        .set_height(height)
        .build();

    view.load_url("http://127.0.0.1:2137")
        .await
        .expect("Failed to load URL");

    let mut interval = time::interval(Duration::from_millis(40));

    loop {
        update_and_render(&renderer);

        if let Err(err) = render_bitmap_and_send(&mut view, &mut device).await {
            eprintln!("Failed to render bitmap: {}", err);
        }

        interval.tick().await;
    }
}
