use self::{
    events::handle_events,
    tray::{build_tray, tray_event_handler},
};
use super::{rendering::message::RendererMessage, sensors::SensorMessage};
use crate::lifecycle::setup;
use smol_static::ServerMessage;
use tachyonix::{Receiver, Sender};
use tauri_plugin_context_menu::init as context_menu_init;
use tauri_plugin_log::{Builder as LogBuilder, LogTarget};
use tauri_plugin_window_state::Builder as WindowStateBuilder;
use tokio::task::{self, JoinHandle};

mod events;
mod main_window;
mod tray;

pub async fn spawn_app(
    sender_renderer: Sender<RendererMessage>,
    sender_sensors: Sender<SensorMessage>,
    receiver_sensors: Receiver<SensorMessage>,
    sender_server: Sender<ServerMessage>,
) -> JoinHandle<Result<(), tauri::Error>> {
    task::spawn_blocking(|| {
        tauri::Builder::default()
            // .plugin(WindowStateBuilder::default().build())
            // .plugin(
            //     LogBuilder::default()
            //         .targets([LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview])
            //         .build(),
            // )
            // .plugin(context_menu_init())
            // .manage(sender_renderer)
            // .manage(sender_sensors)
            // .manage(receiver_sensors)
            // .manage(sender_server)
            // .system_tray(build_tray())
            // .on_system_tray_event(tray_event_handler)
            // .setup(setup)
            .invoke_handler(tauri::generate_handler![crate::lifecycle::exit_handler])
            .any_thread()
            .build(tauri::generate_context!())
            .map(|app| {
                app.run(handle_events);
            })
    })
}
