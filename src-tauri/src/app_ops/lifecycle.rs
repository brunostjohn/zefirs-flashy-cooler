use std::{error::Error, thread, time::Duration};

use tauri::{App, Manager};

use window_shadows::set_shadow;
use window_vibrancy::apply_mica;

#[path = "../config/ensure_dirs.rs"]
mod ensure_dirs;

use crate::{CONFIG, RENDERER, SERVER, THEMES_PATH};

use self::ensure_dirs::ensure_dirs;

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

    let mut server = match SERVER.lock() {
        Ok(result) => result,
        Err(_) => return,
    };

    renderer.stop();
    server.stop();

    std::process::exit(0);
}

#[allow(dead_code)]
pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let renderer = RENDERER.lock().unwrap();
    let config = CONFIG.lock().unwrap();
    let fps = config.fps;

    ensure_dirs();

    let server = SERVER.lock().unwrap();
    match &config.theme_path {
        Some(theme) => {
            let mut full_path = THEMES_PATH.clone();
            full_path.push(theme);
            server.serve_path(Some(full_path));
        }
        None => {
            server.serve_path(None);
        }
    };

    thread::sleep(Duration::from_millis(10));
    renderer.serve();

    renderer.change_fps(fps);

    let window = app.get_window("main").unwrap();

    #[cfg(target_os = "windows")]
    apply_mica(&window).expect("Unsupported platform! 'apply_blur' is only supported on Windows");

    set_shadow(&window, true).unwrap();

    Ok(())
}
