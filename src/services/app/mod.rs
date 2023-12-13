use tokio::task::{self, JoinHandle};

pub async fn spawn_app() -> JoinHandle<Result<(), tauri::Error>> {
    task::spawn_blocking(|| {
        tauri::Builder::default()
            .setup(crate::lifecycle::setup)
            .invoke_handler(tauri::generate_handler![crate::lifecycle::exit_handler])
            .any_thread()
            .run(tauri::generate_context!())
    })
}
