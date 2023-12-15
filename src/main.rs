#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tokio::runtime::Handle;

mod lifecycle;
mod services;
mod themes;
mod utils;
use crate::services::spawn_services;

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(Handle::current());

    let (local, renderer, server, sensors, app) = spawn_services().await;

    let _ = tokio::join!(local, renderer, server, sensors, app);
}
