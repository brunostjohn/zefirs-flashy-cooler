use crate::services::{rendering::message::RendererMessage, sensors::SensorMessage};
use smol_static::ServerMessage;
use tachyonix::Sender;
use tauri::{AppHandle, State, Window};

#[tauri::command]
pub fn exit_handler(
    window: Window,
    app: AppHandle,
    sender_renderer: State<'_, Sender<RendererMessage>>,
    sender_sensors: State<'_, Sender<SensorMessage>>,
    sender_server: State<'_, Sender<ServerMessage>>,
) {
    exit(
        Some(window),
        &app,
        sender_renderer,
        sender_sensors,
        sender_server,
    );
}

pub fn exit(
    window: Option<Window>,
    app: &AppHandle,
    sender_renderer: State<'_, Sender<RendererMessage>>,
    sender_sensors: State<'_, Sender<SensorMessage>>,
    sender_server: State<'_, Sender<ServerMessage>>,
) {
    if let Some(window) = window {
        let _ = window.close();
    }

    let _ = sender_renderer.try_send(RendererMessage::Shutdown);
    let _ = sender_sensors.try_send(SensorMessage::Shutdown);
    let _ = sender_server.try_send(ServerMessage::Shutdown);

    app.exit(0);
}
