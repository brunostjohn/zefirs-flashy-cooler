use anyhow::Context;
use async_compression::tokio::write::GzipEncoder;
use std::path::Path;
use tokio::io::AsyncWriteExt;

pub(crate) struct HTTPFile {
    pub(crate) content_type: String,
    pub(crate) content: Vec<u8>,
    pub(crate) content_length: usize,
}

pub(crate) async fn read_http_file(filename: &str, path: &Path) -> anyhow::Result<HTTPFile> {
    let mut path = path.to_path_buf();

    filename
        .split('/')
        .flat_map(|s| s.split('\\'))
        .filter(|s| !s.starts_with(".."))
        .for_each(|piece| path.push(piece));

    let contents = tokio::fs::read(path)
        .await
        .context("Failed to read file!")?;

    let mut encoder = GzipEncoder::new(Vec::with_capacity(contents.len()));
    encoder
        .write_all(&contents)
        .await
        .context("Failed to compress file!")?;
    encoder.flush().await.context("Failed to flush encoder!")?;
    encoder
        .shutdown()
        .await
        .context("Failed to shutdown encoder!")?;

    let content_type = mime_guess::from_path(filename)
        .first_or_octet_stream()
        .essence_str()
        .to_owned();

    let encoded = encoder.into_inner();
    let content_length = encoded.len();

    Ok(HTTPFile {
        content_type,
        content: encoded,
        content_length,
    })
}
