mod config;
mod dispatch_sensors;
mod r#loop;
pub mod message;
mod render_helpers;
mod setup;
use self::message::RendererMessage;
pub use self::r#loop::main_loop;
use tachyonix::Sender;
use tokio::task::{JoinHandle, LocalSet};

pub fn spawn_renderer(local: &LocalSet) -> (Sender<RendererMessage>, JoinHandle<()>) {
    let (sender, receiver) = tachyonix::channel(10);

    (
        sender,
        local.spawn_local(async move {
            main_loop(receiver).await;
        }),
    )
}
