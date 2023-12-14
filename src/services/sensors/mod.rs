mod r#impl;
mod message;
pub use message::SensorMessage;
use r#impl::*;
use tachyonix::Sender;
use tokio::{
    task::{JoinHandle, LocalSet},
    time::Duration,
};

pub async fn spawn_sensors(
    local: &LocalSet,
    interval: Duration,
) -> (Sender<SensorMessage>, JoinHandle<()>) {
    let (sender, receiver) = tachyonix::channel(10);

    let handle = local.spawn_local(async move {
        let mut sensors = Sensors::new(interval, receiver);
        sensors.run().await;
    });

    (sender, handle)
}
