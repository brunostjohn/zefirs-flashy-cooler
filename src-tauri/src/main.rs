#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use lazy_static::lazy_static;

#[path = "rendering/threading.rs"]
mod rendering;

#[path = "config/config.rs"]
mod config;

#[path = "frontend/basic_ops.rs"]
mod frontend_commands;
use frontend_commands::remote_exit;

use config::Config;

#[path = "app_ops/lifecycle.rs"]
mod lifecycle;
use lifecycle::setup;

#[path = "frontend/tray.rs"]
mod tray;
use tray::{build_tray, tray_event_handler};

use std::{
    env,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use tauri::SystemTray;

use rendering::Renderer;

lazy_static! {
    pub static ref RENDERER: Arc<Mutex<Renderer>> = Arc::new(Mutex::new(Renderer::new(25)));
}

lazy_static! {
    pub static ref APP_FOLDER: PathBuf = match env::current_exe() {
        Ok(mut path) => {
            path.pop();
            path
        }
        _ => PathBuf::from("./"),
    };
}

lazy_static! {
    pub static ref CONFIG: Mutex<Config> = Mutex::new(Config::load_from_drive());
}

fn main() {
    let config = CONFIG.lock().unwrap();

    let start_minimised = config.start_minimised;

    drop(config);

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

    println!("App exited");
}
