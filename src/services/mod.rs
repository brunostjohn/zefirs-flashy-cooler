use self::config::AppConfig;
use crate::utils::themes::paths::get_default_theme_path;
use std::future::Future;
use tokio::task::{JoinHandle, LocalSet};
use tachyonix::Sender;
use smol_static::ServerMessage;

mod app;
mod config;
mod rendering;
mod sensors;
mod server;

pub async fn spawn_services() -> (
    LocalSet,
    JoinHandle<()>,
    impl Future<Output = (Sender<ServerMessage>, JoinHandle<Result<(), anyhow::Error>>)>,
    impl Future<Output = JoinHandle<Result<(), tauri::Error>>>,
) {
    let config = AppConfig::load();
    let server = server::spawn_server(
        config.theme_path.unwrap_or(get_default_theme_path()),
        config.port,
    );
    let local = LocalSet::new();
    let rendering = rendering::spawn_renderer(&local);
    let app = app::spawn_app();

    (local, rendering, server, app)
}
