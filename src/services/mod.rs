use self::config::AppConfig;
use crate::utils::themes::paths::get_default_theme_path;
use tokio::{
    task::{JoinHandle, LocalSet},
    time::Duration,
};

pub mod app;
pub mod config;
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
        config.theme_path.unwrap_or(get_default_theme_path()),
        config.port,
    )
    .await;
    let local = LocalSet::new();
    let interval = Duration::from_millis(config.sensor_poll_rate_ms);
    let (sender_renderer, rendering) = rendering::spawn_renderer(&local);
    let (sender_sensors, receiver_sensors, sensors) =
        sensors::spawn_sensors(&local, interval, sender_renderer.clone()).await;
    let app = app::spawn_app(
        sender_renderer,
        sender_sensors,
        receiver_sensors,
        sender_server,
    )
    .await;

    (local, rendering, sensors, server, app)
}
