#[path = "./ultralight/engine.rs"]
mod engine;

#[path = "devices/device.rs"]
mod device;

use engine::Ultralight;
use image::{self, RgbImage};
use serde::{Deserialize, Serialize};

use std::fs::{self};
use std::path::PathBuf;
use std::sync::{mpsc, Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime};
use std::vec;

use crate::rendering::device::DeviceContainer;
use crate::sensors::Sensors;
use crate::server::Server;

pub struct Renderer {
    thread: Option<JoinHandle<()>>,
    end_channel: mpsc::SyncSender<bool>,
    theme_channel: mpsc::Sender<bool>,
    fps_channel: mpsc::Sender<u64>,
    reload_config_channel: mpsc::Sender<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ThemeConfigItem {
    pub r#type: String,
    pub value: String,
    pub name: String,
}

impl Renderer {
    pub fn new(
        fps: u64,
        app_folder: PathBuf,
        themes_path: PathBuf,
        server: Arc<Mutex<Server>>,
        sensors: Arc<Mutex<Sensors>>,
    ) -> Self {
        let (tx_theme, rx_theme) = mpsc::channel();
        let (tx_end, rx_end) = mpsc::sync_channel(2);
        let (tx_fps, rx_fps) = mpsc::channel();
        let (tx_reload, rx_reload) = mpsc::channel();

        let render = thread::spawn(move || {
            let mut engine = Ultralight::new(app_folder);

            println!("Received {:?} fps", fps);

            static GC_TIMING: Duration = Duration::from_secs(15);

            let mut frame_time = Duration::from_millis(1000 / fps);

            let mut current_time = SystemTime::now();

            let mut gc_time = SystemTime::now();

            let mut sensor_time = SystemTime::now();

            let mut device = match DeviceContainer::new() {
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

            let mut sensor_flag = false;
            let mut sensor_values = vec![];

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
                    if (match sensor_time.elapsed() {
                        Ok(time) => {
                            if time >= Duration::from_millis(600) {
                                sensor_time = SystemTime::now();
                                true
                            } else {
                                false
                            }
                        }
                        Err(_) => false,
                    } && sensor_flag)
                    {
                        let sensors = sensors.lock().unwrap();
                        match sensors.get_sensor_value() {
                            Ok(result) => {
                                if result[0].value != "a"
                                    && result[0].r#type != "a"
                                    && result[0].value != "3"
                                {
                                    sensor_values = result;
                                }
                            }
                            Err(_) => {}
                        };
                        drop(sensors);

                        let mut all_sensor_string = "{".to_owned();

                        for sensor in &sensor_values {
                            all_sensor_string += &("\"".to_owned() + &sensor.code_name + "\":");
                            let sensor_string = serde_json::to_string(&sensor)
                                .or::<Result<String, &'static str>>(Ok("{}".to_owned()))
                                .unwrap();
                            all_sensor_string += &sensor_string;
                            all_sensor_string += ",";
                        }

                        all_sensor_string.pop();
                        all_sensor_string += "}";

                        engine.call_js_script(
                            format!("document.dispatchEvent(new CustomEvent('sensorUpdate', {{ detail: JSON.parse('{}') }}))", &all_sensor_string),
                        );
                    }

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
                }

                if match rx_reload.try_recv() {
                    Ok(result) => result,
                    Err(_) => false,
                } {
                    let server = server.lock().unwrap();
                    let now_serving = server.now_serving();
                    drop(server);
                    let mut theme_path = themes_path.clone();
                    theme_path.push(now_serving);
                    theme_path.push("config.json");

                    if theme_path.exists() {
                        let theme_config_unparsed = fs::read_to_string(theme_path)
                            .or::<Result<String, &'static str>>(Ok("".to_owned()))
                            .unwrap();

                        let theme_config_parsed: Vec<ThemeConfigItem> =
                            serde_json::from_str(&theme_config_unparsed)
                                .or::<Vec<ThemeConfigItem>>(Ok(vec![]))
                                .unwrap();

                        let theme_config = theme_config_parsed;

                        let sensors_only: Vec<ThemeConfigItem> = theme_config
                            .iter()
                            .filter(|x| x.r#type == "sensor")
                            .map(|x| x.to_owned())
                            .collect::<Vec<ThemeConfigItem>>();

                        if sensors_only.len() > 0 {
                            sensor_flag = true;
                            let sensor_paths: Vec<String> =
                                sensors_only.iter().map(|x| x.value.clone()).collect();

                            let sensor_names: Vec<String> =
                                sensors_only.iter().map(|x| x.name.clone()).collect();

                            let mut sensors = sensors.lock().unwrap();

                            sensors.subscribe(sensor_paths, sensor_names);

                            drop(sensors);
                        } else {
                            sensor_flag = false;
                        }

                        let everything_else: Vec<ThemeConfigItem> = theme_config
                            .iter()
                            .filter(|x| x.r#type != "sensor")
                            .map(|x| x.to_owned())
                            .collect::<Vec<ThemeConfigItem>>();

                        let mut collective_sensors_with_labels = "{".to_owned();

                        for item in everything_else {
                            collective_sensors_with_labels +=
                                &("\"".to_owned() + &item.name.clone() + "\":");
                            let item_string = serde_json::to_string::<ThemeConfigItem>(&item)
                                .or::<Result<String, &'static str>>(Ok("[]".to_string()))
                                .unwrap();
                            collective_sensors_with_labels += &item_string;
                            collective_sensors_with_labels += ",";
                        }

                        collective_sensors_with_labels.pop();
                        collective_sensors_with_labels += "}";

                        engine.call_js_script(
                            format!("document.dispatchEvent(new CustomEvent('configLoaded', {{ detail: JSON.parse('{}') }}))", &collective_sensors_with_labels),
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
            reload_config_channel: tx_reload,
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
        self.reload_theme_config();
    }

    pub fn reload_theme_config(&self) {
        match self.reload_config_channel.send(true) {
            Err(_) => {
                println!("Failed to request config reload!");
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
