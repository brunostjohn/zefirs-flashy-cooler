use super::SensorMessage;
use librehardwaremonitor_rs::{Computer, ComputerParams, Hardware, Sensor};
use tachyonix::Receiver;
use tokio::time::{self, Duration};

pub struct Sensors<'a> {
    computer: Computer,
    subscribed_hardware: Vec<Hardware<'a>>,
    subscribed_sensors: Vec<Sensor<'a>>,
    interval: Duration,
    receiver: Receiver<SensorMessage>,
}

impl<'a> Sensors<'a> {
    pub fn new(interval: Duration, receiver: Receiver<SensorMessage>) -> Self {
        let computer = Computer::new_with_params(ComputerParams {
            is_psu_enabled: true,
            is_gpu_enabled: true,
            is_cpu_enabled: true,
            is_motherboard_enabled: true,
            is_memory_enabled: true,
            is_storage_enabled: true,
            is_network_enabled: true,
            is_controller_enabled: true,
            is_battery_enabled: true,
        });
        Self {
            computer,
            subscribed_hardware: Vec::new(),
            subscribed_sensors: Vec::new(),
            interval,
            receiver,
        }
    }

    pub async fn run(&mut self) {
        let mut ticker = time::interval(self.interval);
        loop {
            ticker.tick().await;
            println!("Sensors");
        }
    }
}
