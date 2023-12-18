use tauri::{AppHandle, WindowBuilder};
use tauri_plugin_window_state::{StateFlags, WindowExt};
use window_shadows::set_shadow;

pub fn recreate_main_window(app: &AppHandle) {
    let window =
        WindowBuilder::from_config(app, app.config().tauri.windows.first().unwrap().clone())
            .build()
            .expect("Failed to build window!");

    let _ = window.restore_state(StateFlags::all());

    set_shadow(&window, true).expect("Failed to set window shadows!");
}
