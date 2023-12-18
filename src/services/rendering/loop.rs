use super::{
    config::load_theme_with_config,
    message::{RendererMessage, SensorSubscriptionNotification},
    render_helpers::{render_bitmap_and_send, update_and_render},
    setup::setup_rendering, dispatch_sensors::dispatch_sensors,
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

    let mut subscribed_sensors = vec![];

    loop {
        update_and_render(&renderer);
        if !handle_messages(&mut receiver, &mut view, &mut subscribed_sensors).await {
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
    subscribed_sensors: &mut Vec<SensorSubscriptionNotification>,
) -> bool {
    let received = receiver.try_recv();

    if let Ok(message) = received {
        match message {
            RendererMessage::Shutdown => {
                return false;
            }
            RendererMessage::ReloadCurrentUrl(fs_name) => {
                let _ = load_theme_with_config(view, &fs_name).await;
            }
            RendererMessage::NewSubscribedSensors(sensors) => {
                *subscribed_sensors = sensors;
            }
            RendererMessage::SensorValues(values) => {
                for value in values {
                    if let Some(subscription) = subscribed_sensors
                        .iter_mut()
                        .find(|subscription| subscription.sensor_id == value.sensor_id)
                    {
                        subscription.sensor_value = value.sensor_value;
                    }
                }

                dispatch_sensors(view, subscribed_sensors);
            }
        };

        true
    } else {
        !matches!(received, Err(TryRecvError::Closed))
    }
}
