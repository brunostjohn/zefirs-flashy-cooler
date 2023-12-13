use anyhow::Error;
use smol_static::{Server, ServerMessage};
use std::path::Path;
use tachyonix::Sender;
use tokio::task::JoinHandle;

pub async fn spawn_server<P: AsRef<Path>>(
    base_path: P,
    port: usize,
) -> (Sender<ServerMessage>, JoinHandle<Result<(), Error>>) {
    let (sender, mut server) = Server::new(base_path);
    let handle = tokio::spawn(async move { server.run(port).await });

    (sender, handle)
}
