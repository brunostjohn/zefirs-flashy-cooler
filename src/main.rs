// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod lifecycle;
mod services;

#[tokio::main]
async fn main() {
    tauri::async_runtime::set(tokio::runtime::Handle::current());

    let (rendering) = services::spawn_services();

    tauri::Builder::default()
        .setup(lifecycle::setup)
        .invoke_handler(tauri::generate_handler![lifecycle::exit_handler])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    let (renderer_res, ..) = tokio::join!(rendering);
    renderer_res.expect("Failed to join renderer thread");
}
