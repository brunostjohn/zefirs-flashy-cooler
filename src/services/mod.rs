use self::config::AppConfig;
use crate::utils::themes::paths::get_default_theme_path;
use tokio::{
    task::{JoinHandle, LocalSet},
    time::Duration,
};

mod app;
mod config;
mod rendering;
mod sensors;
mod server;

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
    let (sender_sensors, sensors) = sensors::spawn_sensors(&local, interval).await;
    let rendering = rendering::spawn_renderer(&local);
    let app = app::spawn_app().await;

    (local, rendering, sensors, server, app)
}
