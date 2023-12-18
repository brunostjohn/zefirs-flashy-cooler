mod r#impl;
mod message;
pub use message::SensorMessage;
use r#impl::*;
use tachyonix::{Receiver, Sender};
use tokio::{
    task::{JoinHandle, LocalSet},
    time::Duration,
};

use super::rendering::message::RendererMessage;

pub async fn spawn_sensors(
    local: &LocalSet,
    interval: Duration,
    sender_renderer: Sender<RendererMessage>,
) -> (
    Sender<SensorMessage>,
    Receiver<SensorMessage>,
    JoinHandle<()>,
) {
    let (sender_from, receiver_from) = tachyonix::channel(10);
    let (sender_to, receiver_to) = tachyonix::channel(10);

    let handle = local.spawn_local(async move {
        let sensors = Sensors::new(interval, receiver_to, sender_from, sender_renderer);
        sensors.run().await;
    });

    (sender_to, receiver_from, handle)
}
