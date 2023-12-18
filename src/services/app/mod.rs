use self::{
    events::handle_events,
    tray::{build_tray, tray_event_handler},
};
use super::{rendering::message::RendererMessage, sensors::SensorMessage};
use crate::lifecycle::setup;
use discord_sdk::Discord;
use smol_static::ServerMessage;
use std::sync::Arc;
use tachyonix::{Receiver, Sender};
use tauri::Manager;
use tauri_plugin_context_menu::init as context_menu_init;
use tauri_plugin_log::{Builder as LogBuilder, LogTarget};
use tauri_plugin_single_instance::init as single_instance_init;
use tauri_plugin_window_state::Builder as WindowStateBuilder;
use tokio::{
    sync::RwLock,
    task::{self, JoinHandle},
};

mod events;
mod main_window;
mod tray;

pub async fn spawn_app(
    sender_renderer: Sender<RendererMessage>,
    sender_sensors: Sender<SensorMessage>,
    receiver_sensors: Receiver<SensorMessage>,
    sender_server: Sender<ServerMessage>,
    discord: Arc<RwLock<Option<Discord>>>,
) -> JoinHandle<Result<(), tauri::Error>> {
    task::spawn_blocking(|| {
        tauri::Builder::default()
            .plugin(single_instance_init(|app, argv, cwd| {
                println!("{}, {argv:?}, {cwd}", app.package_info().name);
                app.emit_all("single-instance", Payload { args: argv, cwd })
                    .expect("Failed to emit single instance event!");
            }))
            .plugin(WindowStateBuilder::default().build())
            .plugin(
                LogBuilder::default()
                    .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
                    .build(),
            )
            .plugin(context_menu_init())
            .manage(sender_renderer)
            .manage(sender_sensors)
            .manage(receiver_sensors)
            .manage(sender_server)
            .manage(discord)
            .system_tray(build_tray())
            .on_system_tray_event(tray_event_handler)
            .setup(setup)
            .invoke_handler(tauri::generate_handler![
                crate::lifecycle::exit_handler,
                crate::themes::get_all::get_all_themes_handler,
                crate::themes::play::play_theme_handler,
                crate::services::discord::presence::activity_handler
            ])
            .any_thread()
            .build(tauri::generate_context!())
            .map(|app| {
                app.run(handle_events);
            })
    })
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}
