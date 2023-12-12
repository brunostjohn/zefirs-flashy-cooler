mod r#loop;

use tokio::task::{self, JoinHandle};

use self::r#loop::main_loop;

pub fn spawn_renderer() -> JoinHandle<()> {
    task::spawn(async {
        main_loop().await;
    })
}
