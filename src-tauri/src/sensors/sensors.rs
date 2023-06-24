use std::{
    ffi::CString,
    sync::mpsc,
    thread::{self, JoinHandle},
    time::{Duration, SystemTime},
};

use libloading;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Hardware {
    pub name: String,
    pub subhardware: Option<Vec<Subhardware>>,
    pub sensors: Option<Vec<Sensor>>,
}

#[derive(Deserialize, Debug)]
pub struct Subhardware {
    pub name: String,
    pub sensors: Vec<Sensor>,
}

#[derive(Deserialize, Debug)]
pub struct Sensor {
    pub sensor: String,
    pub value: String,
    pub r#type: String,
}

pub struct Sensors {
    thread_handle: Option<JoinHandle<()>>,
    tx_end: mpsc::SyncSender<bool>,
    tx_poll: mpsc::Sender<u64>,
    tx_subscribe: mpsc::Sender<Vec<String>>,
    rx_sensor_list: mpsc::Receiver<Vec<Hardware>>,
    tx_list_rq: mpsc::Sender<bool>,
    rx_sensor_val: mpsc::Receiver<Vec<Sensor>>,
    tx_park: mpsc::Sender<bool>,
}

impl Sensors {
    pub fn new(poll: Option<u64>) -> Self {
        let (tx_end, rx_end) = mpsc::sync_channel(2);
        let (tx_poll, rx_poll) = mpsc::channel();
        let (tx_subscribe, rx_subscribe) = mpsc::channel::<Vec<String>>();
        let (tx_sensor_list, rx_sensor_list) = mpsc::channel();
        let (tx_list_rq, rx_list_rq) = mpsc::channel();
        let (tx_sensor_val, rx_sensor_val) = mpsc::channel();
        let (tx_park, rx_park) = mpsc::channel();

        let sensor_thread = thread::spawn(move || {
            let lib;
            unsafe {
                lib =
                    libloading::Library::new("./resources/LibreHardwareMonitorNative.dll").unwrap();
            }

            let get_all_sensors_symbol: libloading::Symbol<unsafe extern "C" fn() -> *mut i8>;
            unsafe {
                get_all_sensors_symbol = lib.get(b"get_all_sensors").unwrap();
            }

            let get_single_sensor_symbol: libloading::Symbol<
                unsafe extern "C" fn(*mut i8) -> *mut i8,
            >;
            unsafe {
                get_single_sensor_symbol = lib.get(b"get_single_sensor").unwrap();
            }

            let get_multiple_sensors_symbol: libloading::Symbol<
                unsafe extern "C" fn(*mut i8) -> *mut i8,
            >;
            unsafe {
                get_multiple_sensors_symbol = lib.get(b"get_single_sensor").unwrap();
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

            fn get_single_sensor(ptr: *mut i8) -> Sensor {
                let sensor_string;
                unsafe {
                    let strc = CString::from_raw(ptr);

                    sensor_string = match strc.to_str() {
                        Ok(res) => res.to_owned(),
                        Err(_) => "".to_owned(),
                    };
                }
                let sensor: Sensor = match serde_json::from_str(&sensor_string) {
                    Ok(sensor) => sensor,
                    Err(_) => Sensor {
                        sensor: "".to_owned(),
                        value: "".to_owned(),
                        r#type: "".to_owned(),
                    },
                };

                sensor
            }

            fn get_multiple_sensors(ptr: *mut i8) -> Vec<Sensor> {
                let sensor_string;
                unsafe {
                    let strc = CString::from_raw(ptr);

                    sensor_string = match strc.to_str() {
                        Ok(res) => res.to_owned(),
                        Err(_) => "".to_owned(),
                    };
                }
                let sensor: Vec<Sensor> = match serde_json::from_str(&sensor_string) {
                    Ok(sensors) => sensors,
                    Err(_) => vec![Sensor {
                        sensor: "".to_owned(),
                        value: "".to_owned(),
                        r#type: "".to_owned(),
                    }],
                };

                sensor
            }

            let mut poll = Duration::from_millis(poll.or(Some(3000)).unwrap());
            let mut subscribed_multi = false;
            let mut subscribed: String = "".to_owned();

            loop {
                let start_time = SystemTime::now();

                if match rx_end.try_recv() {
                    Ok(val) => val,
                    Err(_) => true,
                } {
                    println!("Received end signal!");
                    break;
                }

                if match rx_park.try_recv() {
                    Ok(val) => val,
                    Err(_) => true,
                } {
                    println!("Received park signal!");
                    thread::park();
                }

                match rx_poll.try_recv() {
                    Ok(received) => poll = Duration::from_millis(received),
                    Err(_) => {}
                }

                if match rx_list_rq.try_recv() {
                    Ok(val) => val,
                    Err(_) => false,
                } {
                    let sensor_list;
                    unsafe {
                        sensor_list = get_all_sensors(get_all_sensors_symbol());
                    }
                    match tx_sensor_list.send(sensor_list) {
                        Ok(_) => {}
                        Err(_) => println!("Failed to send list."),
                    };
                }

                match rx_subscribe.try_recv() {
                    Ok(sub) => {
                        if sub.len() > 1 {
                            subscribed_multi = true;
                            subscribed = sub.join("||").to_owned();
                        } else {
                            subscribed_multi = false;
                            subscribed = sub[0].clone();
                        }
                    }
                    Err(_) => println!("Failed to receive subscription"),
                }

                if subscribed_multi {
                    let vals;
                    let sensor_string = CString::new(subscribed.clone()).unwrap();

                    unsafe {
                        vals = get_multiple_sensors(get_multiple_sensors_symbol(
                            sensor_string.as_ptr() as *mut i8,
                        ));
                    }

                    let _ = tx_sensor_val.send(vals);
                } else {
                    let val;
                    let sensor_string = CString::new(subscribed.clone()).unwrap();

                    unsafe {
                        val = get_single_sensor(get_single_sensor_symbol(
                            sensor_string.as_ptr() as *mut i8
                        ));
                    }

                    let _ = tx_sensor_val.send(vec![val]);
                }

                match start_time.elapsed() {
                    Ok(dur) => {
                        thread::sleep(poll - dur);
                    }
                    Err(_) => {
                        thread::sleep(poll);
                    }
                }
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
            tx_park,
        }
    }

    pub fn stop(&mut self) {
        match self.tx_end.send(true) {
            Err(_) => {
                println!("Failed to send end sensors message.");
                return;
            }
            Ok(_) => {}
        }

        match self.thread_handle.take() {
            Some(thread) => match thread.join() {
                Ok(_) => {}
                Err(_) => {
                    println!("Failed to join sensors thread!");
                }
            },
            None => {}
        }
    }

    pub fn change_poll_rate(&self, poll_rate: u64) {
        match self.tx_poll.send(poll_rate) {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to adjust poll rate!");
            }
        }
    }

    pub fn subscribe(&self, sensor_paths: Vec<String>) {
        match self.tx_subscribe.send(sensor_paths) {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to send subscribe.")
            }
        }
    }

    pub fn get_all_sensors(&self) -> Result<Vec<Hardware>, &'static str> {
        self.tx_list_rq
            .send(true)
            .or(Err("Failed to send request!"))?;

        self.rx_sensor_list
            .recv()
            .or(Err("Failed to receive sensor values."))
    }

    pub fn get_sensor_value(&self) -> Result<Vec<Sensor>, &'static str> {
        self.rx_sensor_val
            .recv()
            .or(Err("Failed to receive value."))
    }

    pub fn pause(&self) -> Result<(), &'static str> {
        self.tx_park.send(true).or(Err("Failed to send park."))
    }

    pub fn unpause(&self) {
        match self.thread_handle.as_ref() {
            Some(thread) => thread.thread().unpark(),
            None => {}
        }
    }
}
