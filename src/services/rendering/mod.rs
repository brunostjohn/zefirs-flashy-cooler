mod r#loop;
mod render_helpers;
mod setup;

use tokio::task::{JoinHandle, LocalSet};

pub use self::r#loop::main_loop;

pub fn spawn_renderer(local: &LocalSet) -> JoinHandle<()> {
    local.spawn_local(async move {
        main_loop().await;
    })
}
