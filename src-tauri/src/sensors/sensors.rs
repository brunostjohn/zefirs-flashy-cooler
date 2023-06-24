use std::{
    ffi::CString,
    sync::mpsc,
    thread::{self, JoinHandle},
};

use libloading;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Hardware {
    pub name: String,
    pub subhardware: Option<Vec<Subhardware>>,
    pub sensors: Option<Vec<Sensor>>,
}

#[derive(Deserialize, Debug)]
struct Subhardware {
    pub name: String,
    pub sensors: Vec<Sensor>,
}

#[derive(Deserialize, Debug)]
struct Sensor {
    pub sensor: String,
    pub value: String,
    pub r#type: String,
}

pub struct Sensors {
    thread_handle: Option<JoinHandle<()>>,
    tx_end: mpsc::SyncSender<bool>,
    tx_poll: mpsc::Sender<u128>,
    tx_subscribe: mpsc::Sender<String>,
    rx_sensor_list: mpsc::Receiver<Vec<Hardware>>,
    tx_list_rq: mpsc::Sender<bool>,
    rx_sensor_val: mpsc::Receiver<Vec<Sensor>>,
}

impl Sensors {
    pub fn new(poll: Option<u128>) -> Self {
        let (tx_end, rx_end) = mpsc::sync_channel(2);
        let (tx_poll, rx_poll) = mpsc::channel();
        let (tx_subscribe, rx_subscribe) = mpsc::channel();
        let (tx_sensor_list, rx_sensor_list) = mpsc::channel();
        let (tx_list_rq, rx_list_rq) = mpsc::channel();
        let (tx_sensor_val, rx_sensor_val) = mpsc::channel();

        let sensor_thread = thread::spawn(move || {
            let lib;
            let func: libloading::Symbol<unsafe extern "C" fn() -> *mut i8>;
            unsafe {
                lib =
                    libloading::Library::new("./resources/LibreHardwareMonitorNative.dll").unwrap();
                func = lib.get(b"get_all_sensors").unwrap();
            }

            fn get_all_sensors(ptr: *mut i8) -> Vec<Hardware> {
                let sensor_string;
                unsafe {
                    let strc = CString::from_raw(ptr);

                    sensor_string = match strc.to_str() {
                        Ok(res) => res.to_owned(),
                        Err(_) => "".to_owned(),
                    };
                }
                let sensors: Vec<Hardware> = match serde_json::from_str(&sensor_string) {
                    Ok(sensors) => sensors,
                    Err(_) => vec![],
                };

                sensors
            }
            unsafe {
                println!("{:?}", get_all_sensors(func()));
            }
        });

        Self {
            thread_handle: Some(sensor_thread),
            tx_end,
            tx_poll,
            tx_subscribe,
            rx_sensor_list,
            tx_list_rq,
            rx_sensor_val,
        }
    }
}
