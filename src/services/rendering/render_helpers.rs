use anyhow::Context;
use lcd_coolers::DisplayCooler;
use ultralight::{ULRenderer, ULView};

pub async fn render_bitmap_and_send(
    view: &mut ULView<'_>,
    device: &mut impl DisplayCooler,
) -> anyhow::Result<()> {
    let mut surface = view.get_surface();
    let mut bitmap = surface.get_bitmap().context("Failed to get bitmap")?;
    bitmap.swap_red_blue_channels()?;
    let pixels = bitmap.lock_pixels().context("Failed to lock pixels");
    if let Ok(ref pixels) = pixels {
        device.send_image(pixels.as_ref()).await?;
    }
    drop(pixels);

    let _ = bitmap.swap_red_blue_channels();

    Ok(())
}

pub fn update_and_render(renderer: &ULRenderer) {
    renderer.update();
    renderer.render();
}
