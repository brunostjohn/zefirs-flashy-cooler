use tauri::{AppHandle, Window};

#[tauri::command]
pub fn exit_handler(window: Window, app: AppHandle) {
    exit(Some(window), &app);
}

pub fn exit(window: Option<Window>, app: &AppHandle) {
    if let Some(window) = window {
        let _ = window.close();
    }

    app.exit(0);
}
