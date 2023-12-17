use crate::services::{rendering::message::RendererMessage, sensors::SensorMessage};
use discord_sdk::Discord;
use smol_static::ServerMessage;
use std::sync::Arc;
use tachyonix::Sender;
use tauri::{AppHandle, State, Window};
use tokio::sync::RwLock;

#[tauri::command]
pub async fn exit_handler(
    window: Window,
    app: AppHandle,
    sender_renderer: State<'_, Sender<RendererMessage>>,
    sender_sensors: State<'_, Sender<SensorMessage>>,
    sender_server: State<'_, Sender<ServerMessage>>,
    discord: State<'_, Arc<RwLock<Option<Discord>>>>,
) -> Result<(), ()> {
    exit(
        Some(window),
        &app,
        sender_renderer,
        sender_sensors,
        sender_server,
        discord,
    )
    .await;

    Ok(())
}

pub async fn exit(
    window: Option<Window>,
    app: &AppHandle,
    sender_renderer: State<'_, Sender<RendererMessage>>,
    sender_sensors: State<'_, Sender<SensorMessage>>,
    sender_server: State<'_, Sender<ServerMessage>>,
    discord: State<'_, Arc<RwLock<Option<Discord>>>>,
) {
    if let Some(window) = window {
        let _ = window.close();
    }

    let _ = sender_renderer.send(RendererMessage::Shutdown).await;
    let _ = sender_sensors.send(SensorMessage::Shutdown).await;
    let _ = sender_server.send(ServerMessage::Shutdown).await;

    if let Some(discord) = discord.write().await.take() {
        let _ = discord.disconnect().await;
    }

    app.exit(0);
}
