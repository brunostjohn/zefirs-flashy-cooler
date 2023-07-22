use std::error::Error;

use tauri::{App, AppHandle, Manager, Window};

use macros::inject_from_handle;

use window_shadows::set_shadow;
use window_vibrancy::{apply_acrylic, apply_mica};

#[allow(dead_code)]
#[inject_from_handle(config, renderer, sensors, server, app_folder)]
pub fn exit(window: &Option<Window>, app: &AppHandle) {
    println!("Attempting exit.");

    if let Some(win) = window {
        let _ = win.close();
    }

    config.write_to_drive(app_folder.0.clone());

    renderer.stop();

    app.exit(0);
}

#[allow(dead_code)]
#[inject_from_handle(config)]
pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let window = app.get_window("main").unwrap();

    // #[cfg(target_os = "windows")]
    // match apply_mica(&window) {
    //     Ok(_) => {}
    //     Err(_) => {
    //         let _ = apply_acrylic(&window, Some((0, 0, 0, 0)));
    //     }
    // };

    set_shadow(&window, true).unwrap();

    if config.start_minimised {
        let _ = window.close();
    }

    Ok(())
}
