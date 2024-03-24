mod config;
mod dispatch_sensors;
mod r#loop;
pub mod message;
mod render_helpers;
mod setup;
use self::message::RendererMessage;
pub use self::r#loop::main_loop;
use tachyonix::{Receiver, Sender};
use tokio::task::{JoinHandle, LocalSet};

use super::sensors::SensorMessage;

pub fn spawn_renderer(
    local: &LocalSet,
    sensor_sender: Sender<SensorMessage>,
    receiver_renderer: Receiver<RendererMessage>,
) -> JoinHandle<()> {
    local.spawn_local(async move {
        main_loop(receiver_renderer, sensor_sender).await;
    })
}
