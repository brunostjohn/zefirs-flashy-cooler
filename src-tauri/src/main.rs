#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[path = "rendering/threading.rs"]
mod rendering;

#[path = "config/config.rs"]
mod config;

#[path = "frontend/basic_ops.rs"]
mod frontend_commands;

#[path = "frontend/themes.rs"]
mod themes;

use config::Config;

#[path = "app_ops/lifecycle.rs"]
mod lifecycle;
use lifecycle::setup;

#[path = "sensors/sensors.rs"]
mod sensors;

#[path = "services/service.rs"]
mod service;

#[path = "frontend/tray.rs"]
mod tray;
use sensors::Hardware;
use tray::{build_tray, tray_event_handler};

#[path = "file_server/server.rs"]
mod server;
use server::Server;

#[path = "frontend/settings.rs"]
mod settings;

#[path = "config/ensure_dirs.rs"]
mod ensure_dirs;

use self::ensure_dirs::ensure_dirs;

use std::{
    env,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use tauri::SystemTray;

use rendering::Renderer;

use crate::sensors::Sensors;

pub type SensorTree = Vec<Hardware>;
pub type ThemesPath = PathBuf;
pub struct AppFolder(PathBuf);

fn main() {
    let app_folder = match env::current_exe() {
        Ok(mut path) => {
            path.pop();
            path
        }
        _ => PathBuf::from("./"),
    };

    let mut themes_path = match tauri::api::path::document_dir() {
        Some(path) => path,
        None => PathBuf::from("./"),
    };
    themes_path.push("Zefir's Flashy Cooler");

    let config = Config::load_from_drive(app_folder.clone());
    config.write_to_drive(app_folder.clone());

    let (sensors, rx_val) = Sensors::new(Some(config.poll_rate));

    let sensor_tree = sensors.get_all_sensors().unwrap();
    let sensor_tree_am = Arc::new(Mutex::new(sensor_tree));

    let sensors_am = Arc::new(Mutex::new(sensors));

    ensure_dirs(themes_path.clone());

    let mut server = Server::new(None);

    match &config.theme_path {
        Some(theme) => {
            let mut full_path = themes_path.clone();
            full_path.push(theme);
            if full_path.exists() {
                server.serve_path(Some(full_path));
            } else {
                server.serve_path(None);
            }
        }
        None => {
            server.serve_path(None);
        }
    };

    let server_am = Arc::new(Mutex::new(server));

    let renderer = Renderer::new(
        config.fps,
        app_folder.clone(),
        themes_path.clone(),
        server_am.clone(),
        sensors_am.clone(),
        rx_val,
    );

    renderer.serve();

    let renderer_am = Arc::new(Mutex::new(renderer));

    let app_folder_am = Arc::new(Mutex::new(AppFolder(app_folder)));
    let config_am = Arc::new(Mutex::new(config));
    let themes_path_am = Arc::new(Mutex::new(themes_path));

    tauri::Builder::default()
        .manage(sensors_am)
        .manage(renderer_am)
        .manage(app_folder_am)
        .manage(themes_path_am)
        .manage(config_am)
        .manage(sensor_tree_am)
        .manage(server_am)
        .invoke_handler(tauri::generate_handler![
            frontend_commands::remote_exit,
            themes::get_all_themes,
            themes::get_theme_folder,
            themes::open_theme_folder,
            themes::apply_theme,
            themes::install_theme,
            themes::uninstall_theme,
            themes::does_theme_exist,
            themes::get_theme,
            themes::now_serving,
            themes::apply_default,
            themes::get_current_theme_parameter,
            themes::apply_theme_parameter,
            themes::select_file_and_save,
            themes::get_all_sensors,
            themes::select_port,
            settings::get_start_minimised,
            settings::set_start_minimised,
            settings::get_start_login,
            settings::set_start_login,
            settings::get_poll_rate,
            settings::set_poll_rate
        ])
        .system_tray(SystemTray::new().with_menu(build_tray()))
        .on_system_tray_event(tray_event_handler)
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(setup)
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
                // println!("Exit requested");
            }
            _ => {
                // println!("{:#?}", event);
            }
        });

    println!("App exited");
}
