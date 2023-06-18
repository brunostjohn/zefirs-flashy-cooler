use std::error::Error;

use tauri::{App, Manager};

use window_shadows::set_shadow;
use window_vibrancy::apply_mica;

use crate::RENDERER;

#[allow(dead_code)]
pub fn exit() {
    let mut renderer = match RENDERER.lock() {
        Ok(result) => result,
        Err(_) => {
            panic!("Failed to lock renderer.");
        }
    };

    renderer.stop();
    std::process::exit(0);
}

#[allow(dead_code)]
pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let window = app.get_window("main").unwrap();

    #[cfg(target_os = "windows")]
    apply_mica(&window).expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    set_shadow(&window, true).unwrap();

    Ok(())
}
