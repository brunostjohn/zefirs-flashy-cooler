// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tokio::{runtime::Handle, task};

mod lifecycle;
mod services;
use crate::services::spawn_services;

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(Handle::current());

    let (local, app) = spawn_services().await;

    tokio::join!(local, app);
}
