use std::{
    ffi::{CStr, CString},
    sync::mpsc,
    thread::{self, JoinHandle},
};

use rayon::prelude::{IndexedParallelIterator, IntoParallelIterator};
use serde::{Deserialize, Serialize};

#[path = "../helpers/threading.rs"]
mod helpers_threading;
use helpers_threading::EventTicker;

use self::helpers_threading::{receive_flag, ChangeFrequency};

#[derive(Debug)]
#[repr(C)]
struct PreSensor {
    pub sensor: *mut i8,
    pub value: *mut i8,
    pub r#type: *mut i8,
    pub parent_hw_type: *mut i8,
}

#[link(name = "bootstrapperdll", kind = "static")]
#[link(name = "Runtime.WorkstationGC", kind = "static")]
#[link(name = "System.Globalization.Native.Aot", kind = "static")]
#[link(name = "System.IO.Compression.Native.Aot", kind = "static")]
#[link(name = "LibreHardwareMonitorNative", kind = "static")]
extern "C" {
    fn open_computer() -> *mut i16;
    fn close_computer(computer: *mut i16);
    fn get_all_sensors(computer: *mut i16) -> *mut i8;
    fn get_single_sensor_ptrs(path: *mut i8, computer: *mut i16) -> PreSensor;
    fn free_mem(ptr: *mut i8);
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

impl SensorWithDetails {
    pub fn new(code_name: String, old: Sensor) -> Self {
        Self {
            sensor: old.sensor,
            value: old.value,
            r#type: old.r#type,
            parent_hw_type: old.parent_hw_type,
            code_name,
        }
    }
}

pub struct Sensors {
    thread_handle: Option<JoinHandle<()>>,
    tx_end: mpsc::SyncSender<bool>,
    tx_poll: mpsc::Sender<u64>,
    tx_subscribe: mpsc::Sender<Vec<String>>,
    rx_sensor_list: mpsc::Receiver<Vec<Hardware>>,
    tx_list_rq: mpsc::Sender<bool>,
    rx_sensor_val: mpsc::Receiver<Vec<Sensor>>,
    sensor_names: Vec<String>,
}

impl Sensors {
    pub fn new(poll: Option<u64>) -> Self {
        let (tx_end, rx_end) = mpsc::sync_channel(2);
        let (tx_poll, rx_poll) = mpsc::channel();
        let (tx_subscribe, rx_subscribe) = mpsc::channel::<Vec<String>>();
        let (tx_sensor_list, rx_sensor_list) = mpsc::channel();
        let (tx_list_rq, rx_list_rq) = mpsc::channel();
        let (tx_sensor_val, rx_sensor_val) = mpsc::sync_channel(0);

        let sensor_thread = thread::spawn(move || {
            let mut poll = EventTicker::new(poll.or(Some(3000)).unwrap());
            let mut subscribed_multi = false;
            let mut subscribed: String = "".to_owned();

            let computer = unsafe { open_computer() };

            loop {
                if receive_flag(&rx_end, false) {
                    println!("Received end signal!");
                    unsafe {
                        close_computer(computer);
                    }
                    break;
                }

                if receive_flag(&rx_list_rq, false) {
                    let sensor_list = get_all_sensors_vec(computer);

                    let _ = tx_sensor_list
                        .send(sensor_list)
                        .or_else(|_| Err(println!("Failed to send sensor list!")));
                }

                poll.change_frequency(&rx_poll);

                match rx_subscribe.try_recv() {
                    Ok(sub) => {
                        if sub.len() > 1 {
                            subscribed_multi = true;
                            subscribed = sub.join("||").to_owned();
                            println!("got {:?}", &subscribed);
                        } else {
                            subscribed_multi = false;
                            subscribed = sub[0].clone();
                        }
                    }
                    Err(_) => {}
                }

                if subscribed_multi {
                    let vals = get_multiple_sensors(
                        subscribed
                            .split("||")
                            .into_iter()
                            .map(|x| x.to_owned())
                            .collect(),
                        computer,
                    );

                    let _ = tx_sensor_val.send(vals);
                } else {
                    let val = get_single_sensor(&subscribed, computer);
                    let _ = tx_sensor_val.send(vec![val]);
                }

                poll.wait_for_next();
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
            sensor_names: vec![],
        }
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
    pub fn subscribe(&mut self, sensor_paths: Vec<String>, sensor_names: Vec<String>) {
        self.sensor_names = sensor_names;
        match self.tx_subscribe.send(sensor_paths) {
            Ok(_) => {}
            Err(_) => {
                println!("Failed to send subscribe.")
            }
        }
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

    #[inline(always)]
    pub fn get_sensor_value(&self) -> Result<Vec<SensorWithDetails>, &'static str> {
        let sensor_pre = self
            .rx_sensor_val
            .try_recv()
            .or(Err("Failed to receive value."))?;

        if self.sensor_names.len() > 10 {
            let mut details = Vec::with_capacity(self.sensor_names.len());

            for sensor in (&sensor_pre).into_iter() {
                let location = (*sensor_pre)
                    .into_par_iter()
                    .position_any(|x| {
                        x.sensor == sensor.sensor
                            && x.parent_hw_type == sensor.parent_hw_type
                            && x.r#type == sensor.r#type
                    })
                    .unwrap();

                let new_thing =
                    SensorWithDetails::new(self.sensor_names[location].clone(), sensor.clone());

                details.push(new_thing);
            }

            Ok(details)
        } else if self.sensor_names.len() > 1 {
            let mut details = Vec::with_capacity(self.sensor_names.len());

            for sensor in (&sensor_pre).into_iter() {
                let location = (*sensor_pre)
                    .into_iter()
                    .position(|x| {
                        x.sensor == sensor.sensor
                            && x.parent_hw_type == sensor.parent_hw_type
                            && x.r#type == sensor.r#type
                    })
                    .unwrap();

                let new_thing =
                    SensorWithDetails::new(self.sensor_names[location].clone(), sensor.clone());

                details.push(new_thing);
            }

            Ok(details)
        } else {
            let mut details = Vec::with_capacity(1);

            let new_thing =
                SensorWithDetails::new(self.sensor_names[0].clone(), sensor_pre[0].clone());

            details.push(new_thing);

            Ok(details)
        }
    }
}

#[inline(always)]
fn get_all_sensors_vec(computer: *mut i16) -> Vec<Hardware> {
    let ptr = unsafe { get_all_sensors(computer) };
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
fn get_single_sensor(sensor_string: &String, computer: *mut i16) -> Sensor {
    let sensor_string = CString::new(sensor_string.to_owned()).unwrap();
    let pre = unsafe { get_single_sensor_ptrs(sensor_string.as_ptr() as *mut i8, computer) };
    let sensor_cstr = unsafe { CStr::from_ptr(pre.sensor).to_str().unwrap().to_owned() };
    let value_cstr = unsafe { CStr::from_ptr(pre.value).to_str().unwrap().to_owned() };
    let type_cstr = unsafe { CStr::from_ptr(pre.r#type).to_str().unwrap().to_owned() };
    let parent_hw_type_cstr = unsafe {
        CStr::from_ptr(pre.parent_hw_type)
            .to_str()
            .unwrap()
            .to_owned()
    };

    unsafe {
        free_mem(pre.parent_hw_type);
        free_mem(pre.sensor);
        free_mem(pre.r#type);
        free_mem(pre.value);
    };

    Sensor {
        sensor: sensor_cstr,
        value: value_cstr,
        r#type: type_cstr,
        parent_hw_type: Some(parent_hw_type_cstr),
    }
}

#[inline(always)]
fn get_multiple_sensors(sensor_paths: Vec<String>, computer_ptr: *mut i16) -> Vec<Sensor> {
    let mut sensors = vec![];

    for sensor in sensor_paths {
        let sensor = get_single_sensor(&sensor, computer_ptr);
        sensors.push(sensor);
    }

    sensors
}
