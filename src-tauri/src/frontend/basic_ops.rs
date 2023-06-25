#[path = "../app_ops/lifecycle.rs"]
mod lifecycle;
use lifecycle::exit;
use tauri::Window;

#[tauri::command]
pub fn remote_exit(window: Window) {
    exit(&window);
}
