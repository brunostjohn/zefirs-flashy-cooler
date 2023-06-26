use tauri::AppHandle;
use window_shadows::set_shadow;
use window_vibrancy::{apply_acrylic, apply_mica};

pub fn recreate_main_window(app: &AppHandle) {
    let window =
        tauri::WindowBuilder::from_config(app, app.config().tauri.windows.get(0).unwrap().clone())
            .build()
            .unwrap();

    #[cfg(target_os = "windows")]
    match apply_mica(&window) {
        Ok(_) => {}
        Err(_) => {
            let _ = apply_acrylic(&window, Some((0, 0, 0, 0)));
        }
    };

    set_shadow(&window, true).unwrap();
}
