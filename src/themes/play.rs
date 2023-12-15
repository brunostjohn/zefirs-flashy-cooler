use crate::{
    services::rendering::message::RendererMessage, utils::themes::paths::get_all_themes_path,
};
use anyhow::Context;
use smol_static::ServerMessage;
use tachyonix::{SendError, Sender};
use tauri::State;

#[tauri::command]
pub async fn play_theme_handler(
    fs_name: String,
    server_sender: State<'_, Sender<ServerMessage>>,
    renderer_sender: State<'_, Sender<RendererMessage>>,
) -> Result<(), String> {
    play_theme(fs_name, server_sender.inner(), renderer_sender.inner())
        .await
}

async fn play_theme<S: AsRef<str>>(
    fs_name: S,
    server_sender: &Sender<ServerMessage>,
    renderer_sender: &Sender<RendererMessage>,
) -> Result<(), String> {
    let path = get_all_themes_path().join(fs_name.as_ref());
    server_sender
        .send(ServerMessage::SetBasePath(path))
        .await
        .or::<String>(Err("Failed to send server message!".into()))?;
    renderer_sender
        .send(RendererMessage::ReloadCurrentUrl)
        .await
        .or::<String>(Err("Failed to send render message!".into()))?;
    Ok(())
}
