use std::future::Future;

use tokio::task::{JoinHandle, LocalSet};

mod app;
mod rendering;
mod server;
mod sensors;

pub async fn spawn_services() -> (
    LocalSet,
    JoinHandle<()>,
    impl Future<Output = JoinHandle<Result<(), tauri::Error>>>,
) {
    let local = LocalSet::new();
    let rendering = rendering::spawn_renderer(&local);
    let app = app::spawn_app();

    (local, rendering, app)
}
