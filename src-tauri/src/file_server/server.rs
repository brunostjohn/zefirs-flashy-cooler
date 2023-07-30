use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
    thread::{self},
};

mod fs;

pub struct Server {
    end_channel: kanal::Sender<bool>,
    path_channel: kanal::Sender<Option<PathBuf>>,
    serving_fs_name: String,
}

#[path = "../helpers/threading.rs"]
mod helpers_threading;
use helpers_threading::receive_flag;

impl Server {
    pub fn new(path: Option<PathBuf>) -> Self {
        let (tx_end, rx_end) = kanal::unbounded();
        let (tx_path, rx_path) = kanal::unbounded::<Option<PathBuf>>();

        let now_serving: String = match &path {
            Some(path) => {
                let local_p = path.clone();

                local_p.iter().last().unwrap().to_str().unwrap().to_owned()
            }
            None => "__DEFAULT__".to_string(),
        };

        let _ = thread::spawn(move || {
            let listener = TcpListener::bind("127.0.0.1:2137").unwrap();
            let mut fs_path = path;

            for stream in listener.incoming() {
                let stream = stream.unwrap();

                if let Ok(Some(path)) = rx_path.try_recv() {
                    fs_path = path;
                };

                match &fs_path {
                    Some(path) => Server::handle_set_path(stream, path),
                    None => Server::handle_default(stream),
                }

                if receive_flag(&rx_end, false) {
                    break;
                }
            }
        });

        Server {
            end_channel: tx_end,
            path_channel: tx_path,
            serving_fs_name: now_serving,
        }
    }

    fn handle_default(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);

        let _: Vec<_> = buf_reader
            .lines()
            .map(|result| match result {
                Ok(res) => res,
                Err(_) => "".to_string(),
            })
            .take_while(|line| !line.is_empty())
            .collect();

        let mut encoder = flate2::write::GzEncoder::new(vec![], flate2::Compression::default());
        encoder.write_all(DEFAULT_HTML.as_bytes()).unwrap();
        let mut compressed_html = encoder.finish().unwrap();
        let length = compressed_html.len();

        let response_header =
            format!("{OK_STATUS}\r\nConnection: close\r\nContent-Encoding: gzip\r\nContent-Length: {length}\r\nContent-Type: text/html\r\n\r\n");

        let mut response = vec![];
        response.extend(response_header.as_bytes().iter());
        response.append(&mut compressed_html);

        stream.write_all(&response).unwrap();
    }

    fn handle_set_path(mut stream: TcpStream, path: &PathBuf) {
        let buf_reader = BufReader::new(&mut stream);
        let base_path = path.clone();

        let request: Vec<_> = buf_reader
            .lines()
            .map(|result| match result {
                Ok(res) => res,
                Err(_) => "".to_string(),
            })
            .take_while(|line| !line.is_empty())
            .collect();

        let filename = request[0].replace("GET /", "").replace(" HTTP/1.1", "");

        let read_file = match fs::read_http_file(&filename, base_path) {
            Some(file) => file,
            None => {
                Server::return_404(stream);
                return;
            }
        };

        let response_header_vec = vec![
            OK_STATUS,
            "Connection: close",
            "Content-Encoding: gzip",
            &read_file.content_length,
            &read_file.content_type,
            "\r\n",
        ];

        let response_header = response_header_vec.join("\r\n");
        let mut response = vec![];
        response.extend(response_header.as_bytes().iter());
        response.extend(read_file.content.iter());

        stream.write_all(&response).unwrap();
    }

    fn return_404(mut stream: TcpStream) {
        let response_header = "HTTP/1.1 404 Not Found\r\nConnection: close\r\n\r\n".to_string();

        stream.write_all(response_header.as_bytes()).unwrap();
    }

    #[allow(dead_code)]
    pub fn stop(&mut self) {
        if self.end_channel.send(true).is_err() {
            println!("Failed to send end signal to thread: server.");
        };
    }

    pub fn serve_path(&mut self, path: Option<PathBuf>) {
        match path {
            Some(path_some) => {
                match self.path_channel.send(Some(path_some.clone())) {
                    Err(_) => {
                        println!("Failed to change serve path.");
                    }
                    _ => {
                        self.serving_fs_name =
                            path_some.file_name().unwrap().to_str().unwrap().to_owned();
                    }
                };
            }
            None => match self.path_channel.send(None) {
                Err(_) => {
                    println!("Failed to change serve path.");
                }
                _ => {
                    self.serving_fs_name = "__DEFAULT__".to_owned();
                }
            },
        }
    }

    pub fn now_serving(&self) -> String {
        self.serving_fs_name.clone()
    }
}

static OK_STATUS: &str = "HTTP/1.1 200 OK";

static DEFAULT_HTML: &str = include_str!("./default_page.html");
