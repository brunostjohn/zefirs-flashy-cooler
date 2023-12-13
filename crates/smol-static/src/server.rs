use anyhow::Context;
use std::path::{Path, PathBuf};
use tachyonix::TryRecvError;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::filesystem::{read_http_file, HTTPFile};

pub enum ServerMessage {
    SetBasePath(PathBuf),
    Shutdown,
}

pub struct Server {
    base_path: PathBuf,
    receiver: tachyonix::Receiver<ServerMessage>,
}

unsafe impl Send for Server {}
unsafe impl Sync for Server {}

impl Server {
    pub fn new<P: AsRef<Path>>(base_path: P) -> (tachyonix::Sender<ServerMessage>, Self) {
        let base_path = base_path.as_ref().to_path_buf();
        let (sender, receiver) = tachyonix::channel(10);

        (
            sender,
            Self {
                base_path,
                receiver,
            },
        )
    }

    pub fn set_base_path<P: AsRef<Path>>(&mut self, base_path: P) {
        self.base_path = base_path.as_ref().to_path_buf();
    }

    fn rx_from_mpsc(&mut self) -> bool {
        if let Ok(message) = self.receiver.try_recv() {
            match message {
                ServerMessage::SetBasePath(base_path) => {
                    self.set_base_path(base_path);
                }
                ServerMessage::Shutdown => {
                    return false;
                }
            }

            true
        } else {
            !matches!(self.receiver.try_recv(), Err(TryRecvError::Closed))
        }
    }

    pub async fn run(&mut self, port: usize) -> anyhow::Result<()> {
        let listener = TcpListener::bind(&format!("127.0.0.1:{port}"))
            .await
            .context("Failed to bind http server!")?;

        loop {
            let (mut socket, _) = listener
                .accept()
                .await
                .context("Failed to accept connection!")?;

            if !self.rx_from_mpsc() {
                break;
            }

            let cloned_base = self.base_path.clone();

            tokio::spawn(async move {
                let base = cloned_base;
                let mut buf = [0; 2048];
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("Failed to read request!");
                let request = String::from_utf8_lossy(&buf[..n]);
                let request = request.split_whitespace().collect::<Vec<_>>();
                let second_node = request.get(1);

                if let Some(path) = second_node {
                    let path = path
                        .trim_start_matches('/');
                    let path = if path.is_empty() { "index.html" } else { path };
                    if let Ok(HTTPFile {
                        content,
                        content_length,
                        content_type,
                    }) = read_http_file(path, &base).await
                    {
                        let response_header = format!(
                                "HTTP/1.1 200 OK\r\nContent-Encoding: gzip\r\nContent-Type: {}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
                                content_type, content_length
                            );
                        let mut response = vec![];
                        response.extend(response_header.as_bytes().iter());
                        response.extend(content.iter());
                        socket
                            .write_all(response.as_slice())
                            .await
                            .expect("Failed to write response!");
                        socket.flush().await.expect("Failed to flush socket!");
                    } else {
                        return_404(socket).await.expect("Failed to write 404!");
                    }
                }
            });
        }

        Ok(())
    }
}

async fn return_404(mut stream: TcpStream) -> anyhow::Result<()> {
    let response_header = "HTTP/1.1 404 Not Found\r\nConnection: close\r\n\r\n".to_string();

    stream
        .write_all(response_header.as_bytes())
        .await
        .context("Failed to close stream!")
}
