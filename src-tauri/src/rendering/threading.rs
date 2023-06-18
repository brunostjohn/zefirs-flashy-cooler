#[path = "./devices/capellix/device.rs"]
mod capellix;
use capellix::Capellix;
#[path = "./ultralight/engine.rs"]
mod engine;
use engine::Ultralight;
use image::{self, RgbImage};

use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime};

pub struct Renderer {
    thread: Option<JoinHandle<()>>,
    end_channel: mpsc::Sender<bool>,
    theme_channel: mpsc::Sender<bool>,
    fps_channel: mpsc::Sender<u64>,
}

impl Renderer {
    pub fn new(fps: u64) -> Self {
        let (tx_theme, rx_theme) = mpsc::channel();
        let (tx_end, rx_end) = mpsc::channel();
        let (tx_fps, rx_fps) = mpsc::channel();

        let render = thread::spawn(move || {
            let engine = Ultralight::new();

            println!("Received {:?} fps", fps);

            let mut frame_time = Duration::from_millis(1000 / fps);

            let mut current_time = SystemTime::now();

            let mut device = match Capellix::new() {
                Err(error) => {
                    println!("{:?}", error);
                    return;
                }
                Ok(result) => result,
            };

            match device.init() {
                Ok(_) => {}
                Err(err) => println!("{:?}", err),
            };

            loop {
                if thread::panicking() {
                    println!("fuck");
                }
                engine.update();
                println!("Renderer exists");
                if match current_time.elapsed() {
                    Ok(time) => {
                        if time >= frame_time {
                            current_time = SystemTime::now();
                            true
                        } else {
                            false
                        }
                    }
                    Err(_) => {
                        current_time = SystemTime::now();
                        false
                    }
                } {
                    println!("Pushing image.");
                    engine.render();
                    let image = match engine.get_bitmap() {
                        Ok(img) => img,
                        Err(err) => {
                            println!("{:?}", err);
                            vec![]
                        }
                    };

                    match RgbImage::from_raw(480, 480, image.to_vec()) {
                        None => {}
                        Some(image) => {
                            match device.send_image(&image) {
                                Ok(_) => {}
                                Err(_) => {
                                    thread::sleep(Duration::from_secs(7));
                                    match device.reopen() {
                                        Ok(_) => {
                                            match device.init() {
                                                Ok(_) => {}
                                                Err(_) => {
                                                    println!("Failed to reinit device.");
                                                    // send message to ui
                                                }
                                            }
                                        }
                                        Err(_) => {
                                            println!("Failed to reconnect to device.");
                                        }
                                    };
                                }
                            };
                        }
                    }
                }
                thread::sleep(Duration::from_millis(3));

                if match rx_theme.recv() {
                    Ok(result) => result,
                    Err(_) => false,
                } {
                    match engine.load_url("http://localhost:80085") {
                        Ok(_) => {}
                        Err(_) => println!("Failed to reload webpage!"),
                    };
                }

                if match rx_end.recv() {
                    Ok(result) => result,
                    Err(_) => {
                        println!("Failed to receive end signal! Thread ending.");
                        false
                    }
                } {
                    println!("Received end signal. Thread: renderer.");
                    match device.close() {
                        Err(_) => println!("Failed to close device!."),
                        _ => {}
                    };
                    break;
                }

                match rx_fps.recv() {
                    Ok(result) => {
                        frame_time = Duration::from_millis(1000 / result);
                    }
                    Err(_) => {}
                }
            }
        });

        Renderer {
            thread: Some(render),
            theme_channel: tx_theme,
            end_channel: tx_end,
            fps_channel: tx_fps,
        }
    }

    pub fn stop(&mut self) {
        match self.end_channel.send(true) {
            Err(_) => {
                println!("Failed to send end rendering message.");
                return;
            }
            _ => {
                println!("Sent end rendering message.")
            }
        };

        match self.thread.take() {
            Some(thread) => {
                match thread.join() {
                    Ok(_) => {}
                    Err(_) => {
                        println!("Failed to join renderer thread.");
                        return;
                    }
                };
            }
            None => {}
        }
    }

    pub fn serve(&self) {
        match self.theme_channel.send(true) {
            Err(_) => {
                println!("Failed to request refresh!");
            }
            _ => {}
        };
    }

    pub fn change_fps(&self, fps: u64) {
        match self.fps_channel.send(fps) {
            Ok(_) => {}
            Err(_) => println!("Failed to change FPS!"),
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        println!("Renderer dropped!");
    }
}
