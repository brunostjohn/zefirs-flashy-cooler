use std::{error::Error, io::Write, net::TcpStream, thread, time::Duration};

use tauri::{App, Manager, Window};

use window_shadows::set_shadow;
use window_vibrancy::{apply_acrylic, apply_mica};

#[path = "../config/ensure_dirs.rs"]
mod ensure_dirs;

use crate::{CONFIG, RENDERER, SENSORS, SERVER, THEMES_PATH};

use self::ensure_dirs::ensure_dirs;

#[allow(dead_code)]
pub fn exit(window: &Window) {
    println!("Attempting exit.");

    let _ = window.close();

    let config = CONFIG.lock().unwrap();
    config.write_to_drive();

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

    renderer.stop();

    let mut sensors = match SENSORS.lock() {
        Ok(result) => {
            println!("Acquired sensor lock!");
            result
        }
        Err(_) => {
            println!("Failed to get sensor lock");
            return;
        }
    };

    sensors.stop();

    let mut server = match SERVER.lock() {
        Ok(result) => {
            println!("Acquired server lock!");
            result
        }
        Err(_) => {
            println!("Failed to lock server!");
            return;
        }
    };

    server.stop();

    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:2137") {
        let _ = stream.write(&[0x00]);
    }

    std::process::exit(0);
}

#[allow(dead_code)]
pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let renderer = RENDERER.lock().unwrap();
    let config = CONFIG.lock().unwrap();
    let fps = config.fps;

    ensure_dirs();

    let mut server = SERVER.lock().unwrap();
    match &config.theme_path {
        Some(theme) => {
            let mut full_path = THEMES_PATH.clone();
            full_path.push(theme);
            if full_path.exists() {
                server.serve_path(Some(full_path));
            } else {
                server.serve_path(None);
            }
        }
        None => {
            server.serve_path(None);
        }
    };

    drop(server);

    thread::sleep(Duration::from_millis(10));
    renderer.serve();

    renderer.change_fps(fps);

    let window = app.get_window("main").unwrap();

    #[cfg(target_os = "windows")]
    match apply_mica(&window) {
        Ok(_) => {}
        Err(_) => {
            let _ = apply_acrylic(&window, Some((0, 0, 0, 0)));
        }
    };

    set_shadow(&window, true).unwrap();

    if config.start_minimised {
        let _ = window.close();
    }

    Ok(())
}
