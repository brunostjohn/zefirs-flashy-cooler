use crate::services::rendering::message::RendererMessage;

use super::SensorMessage;
use librehardwaremonitor_rs::{Computer, ComputerParams, Hardware, Sensor};
use tachyonix::{Receiver, Sender, TryRecvError};
use tokio::time::{self, Duration, Interval};

pub struct Sensors<'a> {
    computer: Computer,
    subscribed_hardware: Vec<Hardware<'a>>,
    subscribed_sensors: Vec<Sensor<'a>>,
    interval: Duration,
    receiver_to: Receiver<SensorMessage>,
    sender_from: Sender<SensorMessage>,
    sender_renderer: Sender<RendererMessage>,
    ticker: Interval,
}

impl<'a> Sensors<'a> {
    pub fn new(
        interval: Duration,
        receiver_to: Receiver<SensorMessage>,
        sender_from: Sender<SensorMessage>,
        sender_renderer: Sender<RendererMessage>,
    ) -> Self {
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
        let ticker = time::interval(interval);
        Self {
            computer,
            subscribed_hardware: Vec::new(),
            subscribed_sensors: Vec::new(),
            interval,
            receiver_to,
            sender_from,
            sender_renderer,
            ticker,
        }
    }

    pub async fn run(&mut self) {
        loop {
            self.ticker.tick().await;
            if !self.handle_messages().await {
                break;
            }
            println!("Sensors");
        }
    }

    async fn handle_messages(&mut self) -> bool {
        let received = self.receiver_to.try_recv();

        if let Ok(message) = received {
            match message {
                SensorMessage::ChangePoll(duration) => {
                    self.interval = duration;
                    self.ticker = time::interval(self.interval);
                }
                SensorMessage::Shutdown => {
                    return false;
                }
            }

            true
        } else {
            !matches!(received, Err(TryRecvError::Closed))
        }
    }
}
