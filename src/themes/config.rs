use crate::utils::themes::config::MergedConfigItem;

#[tauri::command]
pub async fn get_theme_config(fs_name: String) -> Result<MergedConfigItem, String> {
    Err("Not implemented!".into())
}
