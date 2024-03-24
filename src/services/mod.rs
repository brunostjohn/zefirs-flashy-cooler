use self::{config::AppConfig, discord::register::register};
use crate::utils::themes::paths::get_default_theme_path;
use std::sync::Arc;
use tokio::{
    sync::RwLock,
    task::{JoinHandle, LocalSet},
    time::Duration,
};

pub mod app;
pub mod config;
pub mod discord;
pub mod rendering;
pub mod sensors;
pub mod server;

pub async fn spawn_services() -> (
    LocalSet,
    JoinHandle<()>,
    JoinHandle<()>,
    JoinHandle<Result<(), anyhow::Error>>,
    JoinHandle<Result<(), tauri::Error>>,
) {
    let config = AppConfig::load();
    let (sender_server, server) = server::spawn_server(
        config
            .theme_path
            .as_ref()
            .map(|x| x.to_owned())
            .unwrap_or(get_default_theme_path()),
        config.port,
    )
    .await;
    let local = LocalSet::new();
    let interval = Duration::from_millis(config.sensor_poll_rate_ms);
    let (sender_renderer, receiver_renderer) = tachyonix::channel(10);
    let (sender_sensors, receiver_sensors, sensors) =
        sensors::spawn_sensors(&local, interval, sender_renderer.clone()).await;
    let rendering = rendering::spawn_renderer(&local, sender_sensors.clone(), receiver_renderer);
    let discord = register().ok();
    let app = app::spawn_app(
        sender_renderer,
        sender_sensors,
        receiver_sensors,
        sender_server,
        Arc::new(RwLock::new(discord)),
    )
    .await;

    (local, rendering, sensors, server, app)
}
