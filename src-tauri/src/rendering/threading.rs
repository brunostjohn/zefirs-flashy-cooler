#[path = "./devices/capellix/device.rs"]
mod capellix;
use capellix::Capellix;
#[path = "./ultralight/engine.rs"]
mod engine;
use engine::Ultralight;
use image::{self, RgbImage};
use serde::{Deserialize, Serialize};

use std::fs::{self};
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime};
use std::vec;

use crate::{SENSORS, SERVER, THEMES_PATH};

pub struct Renderer {
    thread: Option<JoinHandle<()>>,
    end_channel: mpsc::SyncSender<bool>,
    theme_channel: mpsc::Sender<bool>,
    fps_channel: mpsc::Sender<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeConfigItem {
    pub r#type: String,
    pub value: String,
}

impl Renderer {
    pub fn new(fps: u64) -> Self {
        let (tx_theme, rx_theme) = mpsc::channel();
        let (tx_end, rx_end) = mpsc::sync_channel(2);
        let (tx_fps, rx_fps) = mpsc::channel();

        let render = thread::spawn(move || {
            let mut engine = Ultralight::new();

            println!("Received {:?} fps", fps);

            static GC_TIMING: Duration = Duration::from_secs(15);

            let mut frame_time = Duration::from_millis(1000 / fps);

            let mut current_time = SystemTime::now();

            let mut gc_time = SystemTime::now();

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

            let mut theme_config_path = PathBuf::new();
            let mut theme_config: Vec<ThemeConfigItem> = vec![];

            loop {
                engine.update();

                if match gc_time.elapsed() {
                    Ok(time) => {
                        if time >= GC_TIMING {
                            gc_time = SystemTime::now();
                            true
                        } else {
                            false
                        }
                    }
                    Err(_) => false,
                } {
                    engine.garbage_collect();
                }

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

                if match rx_theme.try_recv() {
                    Ok(result) => result,
                    Err(_) => false,
                } {
                    match engine.load_url("http://127.0.0.1:2137/") {
                        Ok(_) => {}
                        Err(_) => println!("Failed to reload webpage!"),
                    };

                    if theme_config.len() > 0 {
                        let theme_config_str =
                            serde_json::to_string::<Vec<ThemeConfigItem>>(&theme_config)
                                .or::<Result<String, &'static str>>(Ok("[]".to_owned()))
                                .unwrap();

                        let _ = fs::write(&theme_config_path, theme_config_str);
                    }

                    let server = SERVER.lock().unwrap();
                    let now_serving = server.now_serving();
                    drop(server);
                    let mut theme_path = THEMES_PATH.clone();
                    theme_path.push(now_serving);
                    theme_path.push("config.json");

                    if theme_path.exists() {
                        theme_config_path = theme_path.clone();

                        let theme_config_unparsed = fs::read_to_string(theme_path)
                            .or::<Result<String, &'static str>>(Ok("".to_owned()))
                            .unwrap();

                        let theme_config_parsed: Vec<ThemeConfigItem> =
                            serde_json::from_str(&theme_config_unparsed)
                                .or::<Vec<ThemeConfigItem>>(Ok(vec![]))
                                .unwrap();

                        theme_config = theme_config_parsed;

                        let sensors_only: Vec<String> = theme_config
                            .iter()
                            .filter(|x| x.r#type == "sensor")
                            .map(|x| x.value.clone())
                            .collect();

                        if sensors_only.len() > 0 {
                            let sensors = SENSORS.lock().unwrap();
                            sensors.subscribe(sensors_only);
                            drop(sensors);
                        }

                        let everything_else: Vec<ThemeConfigItem> = theme_config
                            .iter()
                            .filter(|x| x.r#type != "sensor")
                            .map(|x| x.to_owned())
                            .collect::<Vec<ThemeConfigItem>>();

                        let everything_else_string =
                            serde_json::to_string::<Vec<ThemeConfigItem>>(&everything_else)
                                .or::<Result<String, &'static str>>(Ok("[]".to_string()))
                                .unwrap();

                        engine.call_js_script(
                        format!("document.dispatchEvent(new CustomEvent('configLoaded', JSON.parse('{:?}')))", &everything_else_string),
                        );
                    }
                }

                if match rx_end.try_recv() {
                    Ok(result) => result,
                    _ => false,
                } {
                    println!("Received end signal. Thread: renderer.");
                    match device.close() {
                        Err(_) => println!("Failed to close device!."),
                        _ => {}
                    };
                    match device.close() {
                        Err(_) => {
                            println!("Failed to close device.");
                        }
                        _ => {}
                    };
                    break;
                }

                match rx_fps.try_recv() {
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
