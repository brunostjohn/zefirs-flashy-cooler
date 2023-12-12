use anyhow::Context;
use std::error::Error;
use tauri::{App, Manager};
use window_shadows::set_shadow;

pub fn setup(app: &mut App) -> Result<(), Box<dyn Error>> {
    let window = app
        .get_window("main")
        .context("Failed to find main window!")?;
    set_shadow(&window, true).expect("Failed to set window shadow");

    Ok(())
}
