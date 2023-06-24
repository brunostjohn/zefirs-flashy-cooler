use crate::CONFIG;

#[tauri::command]
pub fn get_start_minimised() -> bool {
    let config = CONFIG.lock().unwrap();
    return config.start_minimised;
}
