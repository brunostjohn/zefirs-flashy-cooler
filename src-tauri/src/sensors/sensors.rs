use std::{
    ffi::{CStr, CString},
    thread::{self, JoinHandle},
};

use serde::{Deserialize, Serialize};

#[path = "../helpers/threading.rs"]
mod helpers_threading;
use helpers_threading::EventTicker;

use self::helpers_threading::{receive_flag, ChangeFrequency};

#[link(name = "bootstrapperdll", kind = "static")]
#[link(name = "Runtime.WorkstationGC", kind = "static")]
#[link(name = "System.Globalization.Native.Aot", kind = "static")]
#[link(name = "System.IO.Compression.Native.Aot", kind = "static")]
#[link(name = "LibreHardwareMonitorNative", kind = "static")]
extern "C" {
    fn open_computer();
    fn close_computer();
    fn get_all_sensors() -> *mut i8;
    fn get_subscribed_ptr() -> *mut i8;
    fn free_mem(ptr: *mut i8);
    fn subscribe(structs: *mut u8, count: u8) -> *mut i8;
}

#[repr(C, packed)]
pub struct Subscription {
    pub code_name: CString,
    pub path: CString,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Hardware {
    pub name: String,
    pub subhardware: Option<Vec<Subhardware>>,
    pub sensors: Option<Vec<Sensor>>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Subhardware {
    pub name: String,
    pub sensors: Vec<Sensor>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Sensor {
    pub sensor: String,
    pub value: String,
    pub r#type: String,
    pub parent_hw_type: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct SensorWithDetails {
    pub sensor: String,
    pub value: String,
    pub r#type: String,
    pub parent_hw_type: Option<String>,
    pub code_name: String,
}

pub struct Sensors {
    thread_handle: Option<JoinHandle<()>>,
    tx_end: kanal::Sender<bool>,
    tx_poll: kanal::Sender<u64>,
    tx_subscribe: kanal::Sender<(Vec<String>, Vec<String>)>,
    rx_sensor_list: kanal::Receiver<Vec<Hardware>>,
    tx_list_rq: kanal::Sender<bool>,
    rx_subscribed: kanal::Receiver<Vec<SensorWithDetails>>,
}

impl Sensors {
    pub fn new(poll: Option<u64>) -> (Self, kanal::Receiver<String>) {
        let (tx_end, rx_end) = kanal::bounded(2);
        let (tx_poll, rx_poll) = kanal::unbounded();
        let (tx_subscribe, rx_subscribe) = kanal::unbounded::<(Vec<String>, Vec<String>)>();
        let (tx_sensor_list, rx_sensor_list) = kanal::unbounded();
        let (tx_list_rq, rx_list_rq) = kanal::unbounded();
        let (tx_sensor_val, rx_sensor_val) = kanal::unbounded::<String>();
        let (tx_subscribed, rx_subscribed) = kanal::unbounded();

        let sensor_thread = thread::spawn(move || {
            let mut poll = EventTicker::new(poll.unwrap_or(3000));

            unsafe { open_computer() };

            loop {
                if receive_flag(&rx_end, false) {
                    println!("Received end signal!");
                    unsafe {
                        close_computer();
                    }
                    break;
                }

                if receive_flag(&rx_list_rq, false) {
                    let sensor_list = get_all_sensors_vec();

                    let _ = tx_sensor_list
                        .send(sensor_list)
                        .map_err(|_| println!("Failed to send sensor list!"));
                }

                poll.change_frequency(&rx_poll);

                if let Ok(sub_send) = rx_subscribe.try_recv() {
                    if let Some(sub) = sub_send {
                        let (mut paths, mut names) = sub;
                        let sensor_count = paths.len();

                        let sensors;
                        let mut names_copy = names.clone();
                        names.reverse();

                        if sensor_count > 1 {
                            let mut subs: Vec<Subscription> = Vec::with_capacity(paths.len());

                            for _ in 0..sensor_count {
                                let new_sub = Subscription {
                                    code_name: CString::new(names.pop().unwrap()).unwrap(),
                                    path: CString::new(paths.pop().unwrap()).unwrap(),
                                };

                                subs.push(new_sub);
                            }

                            sensors = unsafe {
                                subscribe(subs.as_mut_ptr() as *mut u8, paths.len() as u8)
                            };
                        } else {
                            let mut subs = [Subscription {
                                code_name: CString::new(names.pop().unwrap()).unwrap(),
                                path: CString::new(paths.pop().unwrap()).unwrap(),
                            }];

                            sensors = unsafe { subscribe(subs.as_mut_ptr() as *mut u8, 1) };
                        }

                        let csensors = unsafe { CStr::from_ptr(sensors as *const i8) }
                            .to_str()
                            .unwrap();

                        let mut deets = vec![];

                        for element in csensors.split("****").collect::<Vec<&str>>() {
                            let mut fields: Vec<&str> = element.split("||").collect();

                            if fields.len() == 4 {
                                let with_details = SensorWithDetails {
                                    parent_hw_type: Some(fields.pop().unwrap().to_owned()),
                                    r#type: fields.pop().unwrap().to_owned(),
                                    value: fields.pop().unwrap().to_owned(),
                                    sensor: fields.pop().unwrap().to_owned(),
                                    code_name: names_copy.pop().unwrap(),
                                };

                                deets.push(with_details);
                            }
                        }

                        tx_subscribed.send(deets).unwrap();
                    }
                }

                let _ = tx_sensor_val.send(get_subscribed());

                poll.wait_for_next();
            }
        });

        (
            Self {
                thread_handle: Some(sensor_thread),
                tx_end,
                tx_poll,
                tx_subscribe,
                rx_sensor_list,
                tx_list_rq,
                rx_subscribed,
            },
            rx_sensor_val,
        )
    }

    #[inline(always)]
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

    #[inline(always)]
    pub fn change_poll_rate(&self, poll_rate: u64) {
        match self.tx_poll.send(poll_rate) {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to adjust poll rate!");
            }
        }
    }

    #[inline(always)]
    pub fn subscribe(
        &mut self,
        sensor_paths: Vec<String>,
        sensor_names: Vec<String>,
    ) -> Result<Vec<SensorWithDetails>, &str> {
        self.tx_subscribe
            .send((sensor_paths, sensor_names))
            .or(Err("Failed to send subscribe!"))?;

        self.rx_subscribed
            .recv()
            .or(Err("Failed to get sensor with details back."))
    }

    #[inline(always)]
    pub fn get_all_sensors(&self) -> Result<Vec<Hardware>, &'static str> {
        self.tx_list_rq
            .send(true)
            .or(Err("Failed to send request!"))?;

        self.rx_sensor_list
            .recv()
            .or(Err("Failed to receive sensor values."))
    }
}

#[inline(always)]
fn get_all_sensors_vec() -> Vec<Hardware> {
    let ptr = unsafe { get_all_sensors() };
    let sensor_string;
    unsafe {
        let strc = CStr::from_ptr(ptr);

        sensor_string = match strc.to_str() {
            Ok(res) => res.to_owned(),
            Err(_) => "".to_owned(),
        };

        free_mem(ptr);
    }
    let sensors: Vec<Hardware> = match serde_json::from_str(&sensor_string) {
        Ok(sensors) => sensors,
        Err(_) => vec![],
    };

    sensors
}

#[inline(always)]
fn get_subscribed() -> String {
    let pre = unsafe { get_subscribed_ptr() };

    let cstringified = unsafe { CStr::from_ptr(pre as *const i8) };

    let stringified = cstringified.to_str().unwrap().to_string();

    unsafe {
        free_mem(pre);
    };

    stringified
}
