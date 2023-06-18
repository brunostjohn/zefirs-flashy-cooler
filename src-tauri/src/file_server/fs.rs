use std::{fs, io::Write, path::PathBuf};

pub struct HTTPFile {
    pub content_type: String,
    pub content_length: String,
    pub content: Vec<u8>,
}

pub fn read_http_file(filename: &str, path: PathBuf) -> Option<HTTPFile> {
    let mut path = path.clone();

    let filename = match filename {
        "" => "index.html".to_owned(),
        name => name.to_owned(),
    };

    for separated in filename.split("/") {
        path.push(separated);
    }

    let file = match fs::read(&path) {
        Err(_) => return None,
        Ok(file) => file,
    };

    let mime = match infer::get(&file) {
        Some(kind) => kind.mime_type().to_owned(),
        None => match mime_guess::from_path(path.as_path()).first() {
            Some(kind) => {
                let kind = kind.to_string();
                kind
            }
            None => "application/octet-stream".to_owned(),
        },
    };

    let mut encoder = flate2::write::GzEncoder::new(vec![], flate2::Compression::default());
    encoder.write_all(&file).unwrap();

    let content = encoder.finish().unwrap();
    let length = content.len();
    let content_length = format!("Content-Length: {length}");
    let content_type = format!("Content-Type: {mime}");

    Some(HTTPFile {
        content_type,
        content_length,
        content,
    })
}
