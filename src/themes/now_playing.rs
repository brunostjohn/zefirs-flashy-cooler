use crate::services::config::AppConfig;

#[tauri::command]
pub fn get_now_playing() -> Result<String, String> {
    let config = AppConfig::load();

    config
        .theme_path
        .and_then(|x| x.split('\\').last().map(|x| x.to_string()))
        .ok_or("No theme playing!".into())
}
