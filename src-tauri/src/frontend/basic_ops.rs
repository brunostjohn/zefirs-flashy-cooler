#[path = "../app_ops/lifecycle.rs"]
mod lifecycle;
use lifecycle::exit;
use tauri::{AppHandle, Window};

#[tauri::command]
pub fn remote_exit(window: Window, app: AppHandle) {
    exit(&Some(window), &app);
}
