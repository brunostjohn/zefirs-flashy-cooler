use crate::{
    services::{config::AppConfig, rendering::message::RendererMessage},
    utils::themes::paths::get_all_themes_path,
};
use smol_static::ServerMessage;
use tachyonix::Sender;
use tauri::State;

use super::validate::validate_theme;

#[tauri::command]
pub async fn play_theme_handler(
    fs_name: String,
    server_sender: State<'_, Sender<ServerMessage>>,
    renderer_sender: State<'_, Sender<RendererMessage>>,
) -> Result<(), String> {
    play_theme(fs_name, server_sender.inner(), renderer_sender.inner()).await
}

async fn play_theme<S: AsRef<str>>(
    fs_name: S,
    server_sender: &Sender<ServerMessage>,
    renderer_sender: &Sender<RendererMessage>,
) -> Result<(), String> {
    if !validate_theme(fs_name.as_ref()).await {
        return Err("Invalid theme name!".into());
    }
    let path = get_all_themes_path().join(fs_name.as_ref());
    if let Some(path_str) = path.clone().to_str() {
        let mut config = AppConfig::load();
        config.theme_path = Some(path_str.to_string());
        let _ = config.save();
    }
    server_sender
        .send(ServerMessage::SetBasePath(path))
        .await
        .or::<String>(Err("Failed to send server message!".into()))?;
    renderer_sender
        .send(RendererMessage::ReloadCurrentUrl(
            fs_name.as_ref().to_string(),
        ))
        .await
        .or::<String>(Err("Failed to send render message!".into()))?;
    Ok(())
}
