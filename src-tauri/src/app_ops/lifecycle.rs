use std::error::Error;

use tauri::{App, Manager};

use window_shadows::set_shadow;
use window_vibrancy::apply_mica;

use crate::{CONFIG, RENDERER};

#[allow(dead_code)]
pub fn exit() {
    println!("Attempting exit.");

    let mut renderer = match RENDERER.lock() {
        Ok(result) => {
            println!("Acquired renderer lock.");
            result
        }
        Err(_) => {
            println!("Failed to lock renderer.");
            return;
        }
    };

    // renderer.stop();
    std::process::exit(0);
}

#[allow(dead_code)]
pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let renderer = RENDERER.lock().unwrap();
    let config = CONFIG.lock().unwrap();
    let fps = config.fps;

    renderer.change_fps(fps);

    let window = app.get_window("main").unwrap();

    #[cfg(target_os = "windows")]
    apply_mica(&window).expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    set_shadow(&window, true).unwrap();

    Ok(())
}
