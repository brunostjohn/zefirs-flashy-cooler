#[path = "./ultralight/engine.rs"]
mod engine;

#[path = "devices/device.rs"]
mod device;

#[path = "../helpers/threading.rs"]
mod helpers_threading;
use helpers_threading::receive_flag;

#[path = "../helpers/traits.rs"]
mod traits;
// use traits::{Reassign, TryElapsed};

use engine::Ultralight;
use image::{self, RgbImage};
use serde::{Deserialize, Serialize};

use std::fs::{self};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, SystemTime};
use std::vec;

use heapless::spsc::Queue;

use crate::rendering::device::DeviceContainer;
use crate::rendering::helpers_threading::{ChangeFrequency, EventTicker};
use crate::rendering::traits::CustomSerialise;
use crate::sensors::{SensorWithDetails, Sensors};
use crate::server::Server;

pub struct Renderer {
    thread: Option<JoinHandle<()>>,
    end_channel: kanal::Sender<bool>,
    // theme_channel: kanal::Sender<bool>,
    fps_channel: kanal::Sender<Duration>,
    reload_config_channel: kanal::Sender<bool>,
    port_channel: kanal::Sender<usize>,
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
        rx_sensor: kanal::Receiver<String>,
    ) -> Self {
        // let (tx_theme, rx_theme) = kanal::unbounded();
        let (tx_end, rx_end) = kanal::bounded(2);
        let (tx_fps, rx_fps) = kanal::unbounded();
        let (tx_reload, rx_reload) = kanal::unbounded();
        let (tx_port, rx_port) = kanal::unbounded();

        let render = thread::spawn(move || {
            let mut engine = Ultralight::new(app_folder, &mut Queue::new());

            println!("Received {:?} fps", fps);

            // let mut gc_time = EventTicker::new(15 * 1000);
            let mut frame_time = EventTicker::new(1000 / fps);
            let mut sensor_time = EventTicker::new(3000);
            let mut channel_scan = EventTicker::new(250);
            let mut gc_time = EventTicker::new(1000 * 30);

            let mut device = match DeviceContainer::new() {
                Err(error) => {
                    println!("{:?}", error);
                    return;
                }
                Ok(result) => result,
            };

            let _ = device
                .init()
                .map_err(|_| println!("Failed to initialise device."));

            let mut sensor_flag = false;
            let mut sensor_values: Vec<SensorWithDetails> = vec![];

            thread::sleep(Duration::from_millis(300));

            let _ = engine
                .load_url(&format!("http://127.0.0.1:2137"))
                .map_err(|_| println!("Failed to reload theme!"));

            loop {
                // let time = SystemTime::now();
                engine.update();

                if frame_time.check_time() {
                    if sensor_time.check_time() && sensor_flag {
                        let value = rx_sensor.try_recv().unwrap();

                        if let Some(result) = value {
                            if !result.contains("FAILEDFAILEDFAILED") {
                                let mut iterable = result.split("||");

                                for item in &mut sensor_values {
                                    item.value = iterable.next().unwrap().to_string();
                                }
                            }
                        }

                        let script = format!("document.dispatchEvent(new CustomEvent('sensorUpdate', {{ detail: JSON.parse('{}') }}))", sensor_values.custom_serialise());

                        engine.call_js_script(script);
                    }
                    engine.render();
                    let image = engine.get_bitmap().unwrap();

                    if device.send_image(&image).is_err() {
                        thread::sleep(Duration::from_secs(7));

                        if device.reopen().is_ok() {
                            device.init().unwrap();
                        }
                    }
                    if false && channel_scan.check_time() {
                        if gc_time.check_time() {
                            engine.garbage_collect();
                        }

                        if let Ok(Some(port)) = rx_port.try_recv() {
                            let _ = engine
                                .load_url(&format!("http://127.0.0.1:{port}"))
                                .map_err(|_| println!("Failed to reload theme!"));
                        }

                        if receive_flag(&rx_reload, false) {
                            let server = server.lock().unwrap();
                            let now_serving = server.now_serving();
                            drop(server);
                            let mut theme_path = themes_path.clone();
                            theme_path.push(now_serving);
                            theme_path.push("config.json");

                            if theme_path.exists() {
                                let theme_config_unparsed =
                                    fs::read_to_string(theme_path).unwrap_or("".to_owned());

                                let theme_config_parsed: Vec<ThemeConfigItem> =
                                    serde_json::from_str(&theme_config_unparsed)
                                        .unwrap_or(Vec::new());

                                let sensors_only: Vec<ThemeConfigItem> = theme_config_parsed
                                    .iter()
                                    .filter(|x| x.r#type == "sensor")
                                    .map(|x| x.to_owned())
                                    .collect::<Vec<ThemeConfigItem>>();

                                if !sensors_only.is_empty() {
                                    sensor_flag = true;
                                    let sensor_paths: Vec<String> =
                                        sensors_only.iter().map(|x| x.value.clone()).collect();

                                    let sensor_names: Vec<String> =
                                        sensors_only.iter().map(|x| x.name.clone()).collect();

                                    let mut sensors = sensors.lock().unwrap();

                                    if let Ok(vals) = sensors
                                        .subscribe(sensor_paths.clone(), sensor_names.clone())
                                    {
                                        sensor_values = vals;
                                    }

                                    drop(sensors);
                                } else {
                                    sensor_flag = false;
                                }

                                let serialised = theme_config_parsed.custom_serialise();

                                engine.call_js_script(
                                format!("document.dispatchEvent(new CustomEvent('configLoaded', {{ detail: JSON.parse('{}') }}))", &serialised),
                            );
                            }
                        }

                        if receive_flag(&rx_end, false) {
                            println!("Received end signal. Thread: renderer.");

                            let _ = device
                                .close()
                                .map_err(|_| println!("Failed to close device!"));

                            break;
                        }

                        frame_time.change_frequency(&rx_fps);
                    }
                } else {
                    thread::sleep(Duration::from_millis(10));
                }

                // let elapsed = time
                //     .elapsed()
                //     .unwrap_or(Duration::from_micros(0))
                //     .as_millis();
                // println!("ms: {elapsed}");
            }
        });

        Renderer {
            thread: Some(render),
            end_channel: tx_end,
            fps_channel: tx_fps,
            reload_config_channel: tx_reload,
            port_channel: tx_port,
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
            Some(thread) => match thread.join() {
                Ok(_) => {}
                Err(_) => {
                    println!("Failed to join renderer thread.");
                }
            },
            None => {}
        }
    }

    pub fn serve(&self) {
        match self.port_channel.send(2137) {
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

    #[allow(dead_code)]
    pub fn change_fps(&self, fps: u64) {
        match self.fps_channel.send(Duration::from_millis(1000 / fps)) {
            Ok(_) => {}
            Err(_) => println!("Failed to change FPS!"),
        }
    }

    pub fn send_port(&self, port: usize) {
        match self.port_channel.send(port) {
            Ok(_) => {}
            Err(_) => println!("Failed to send port!"),
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        println!("Renderer dropped!");
    }
}
