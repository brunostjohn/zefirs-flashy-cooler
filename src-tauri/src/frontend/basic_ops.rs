#[path = "../app_ops/lifecycle.rs"]
mod lifecycle;
use lifecycle::exit;
use macros::inject;
use tauri::{AppHandle, Window};

#[inject(server, config, renderer, sensors, app_folder)]
#[tauri::command]
pub fn remote_exit(window: Window, app: AppHandle) {
    exit(&window, &app);
}
