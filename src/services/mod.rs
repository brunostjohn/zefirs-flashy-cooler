use tokio::task::JoinHandle;

mod rendering;

pub fn spawn_services() -> (JoinHandle<()>) {
    let rendering = rendering::spawn_renderer();

    (rendering)
}
