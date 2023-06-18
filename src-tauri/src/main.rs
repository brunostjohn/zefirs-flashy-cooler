#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[path = "rendering/threading.rs"]
mod rendering;

#[path = "frontend/basic_ops.rs"]
mod frontend_commands;
use frontend_commands::remote_exit;

#[path = "app_ops/lifecycle.rs"]
mod lifecycle;
use lifecycle::setup;

#[path = "frontend/tray.rs"]
mod tray;
use tray::{build_tray, tray_event_handler};

use once_cell::sync::Lazy;
use std::sync::Mutex;

use tauri::SystemTray;

use rendering::{build_renderer, Renderer};

pub static RENDERER: Lazy<Mutex<Renderer>> = Lazy::new(|| {
    let renderer = build_renderer(25);

    Mutex::new(renderer)
});

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![remote_exit])
        .system_tray(SystemTray::new().with_menu(build_tray()))
        .on_system_tray_event(tray_event_handler)
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(setup)
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
