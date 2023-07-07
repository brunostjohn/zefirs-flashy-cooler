use std::error::Error;

use tauri::{App, AppHandle, Manager, Window};

use macros::inject_from_handle;

use window_shadows::set_shadow;
use window_vibrancy::{apply_acrylic, apply_mica};

#[inject_from_handle(config, renderer, sensors, server, app_folder)]
#[allow(dead_code)]
pub fn exit(window: &Window, app: &AppHandle) {
    println!("Attempting exit.");

    let _ = window.close();

    config.write_to_drive(app_folder.0.clone());

    renderer.stop();

    sensors.stop();

    app.exit(0);
}

#[allow(dead_code)]
pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let window = app.get_window("main").unwrap();

    #[cfg(target_os = "windows")]
    match apply_mica(&window) {
        Ok(_) => {}
        Err(_) => {
            let _ = apply_acrylic(&window, Some((0, 0, 0, 0)));
        }
    };

    set_shadow(&window, true).unwrap();

    // if config.start_minimised {
    //     let _ = window.close();
    // }

    Ok(())
}
