#[path = "../app_ops/lifecycle.rs"]
mod lifecycle;
use lifecycle::exit;

#[tauri::command]
pub fn remote_exit() {
    exit();
}
