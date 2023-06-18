use tauri::AppHandle;
use window_shadows::set_shadow;
use window_vibrancy::apply_mica;

pub fn recreate_main_window(app: &AppHandle) {
    let window =
        tauri::WindowBuilder::from_config(app, app.config().tauri.windows.get(0).unwrap().clone())
            .build()
            .unwrap();

    apply_mica(&window).unwrap();
    set_shadow(&window, true).unwrap();
}
