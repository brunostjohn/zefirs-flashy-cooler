use super::{
    message::RendererMessage,
    render_helpers::{render_bitmap_and_send, update_and_render},
    setup::setup_rendering,
};
use lcd_coolers::{DeviceCreator, DeviceInfo, DisplayCooler};
use tachyonix::{Receiver, TryRecvError};
use tokio::time::{self, Duration};
use ultralight::{ULView, ULViewBuilder};

pub async fn main_loop(receiver: Receiver<RendererMessage>) {
    let (renderer, mut device) = setup_rendering().await.expect("Failed to setup rendering!");
    let mut receiver = receiver;
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
        if !handle_messages(&mut receiver, &mut view).await {
            break;
        }

        if let Err(err) = render_bitmap_and_send(&mut view, &mut device).await {
            eprintln!("Failed to render bitmap: {}", err);
        }

        interval.tick().await;
    }

    let _ = device.close().await;
}

async fn handle_messages<'a>(
    receiver: &mut Receiver<RendererMessage>,
    view: &mut ULView<'a>,
) -> bool {
    let received = receiver.try_recv();

    if let Ok(message) = received {
        match message {
            RendererMessage::Shutdown => {
                return false;
            }
            RendererMessage::ReloadCurrentUrl => view.reload(),
        };

        true
    } else {
        !matches!(received, Err(TryRecvError::Closed))
    }
}
