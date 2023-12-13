mod r#loop;

use tokio::task::{self, JoinHandle, LocalSet};

pub use self::r#loop::main_loop;

pub fn spawn_renderer(local: &LocalSet) -> JoinHandle<()> {
    local.spawn_local(async move {
        main_loop().await;
    })
}
