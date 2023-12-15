use crate::services::rendering::message::{
    RendererMessage, SensorSubscriptionNotification, SensorValue,
};

use super::{
    message::{SensorsResponse, SubscribeRequest},
    SensorMessage,
};
use librehardwaremonitor_rs::{Computer, ComputerParams};
use tachyonix::{Receiver, Sender, TryRecvError};
use tokio::time::{self, Duration, Interval};

pub struct Sensors {
    last_id: usize,
    subscribed: Vec<(usize, SubscribeRequest)>,
    interval: Duration,
    receiver_to: Receiver<SensorMessage>,
    sender_from: Sender<SensorMessage>,
    sender_renderer: Sender<RendererMessage>,
    ticker: Interval,
    computer: Computer,
}

impl Sensors {
    pub fn new(
        interval: Duration,
        receiver_to: Receiver<SensorMessage>,
        sender_from: Sender<SensorMessage>,
        sender_renderer: Sender<RendererMessage>,
    ) -> Self {
        let ticker = time::interval(interval);
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
            last_id: 0,
            computer,
            subscribed: vec![],
            interval,
            receiver_to,
            sender_from,
            sender_renderer,
            ticker,
        }
    }

    pub async fn run(mut self) {
        loop {
            self.ticker.tick().await;
            if !self.handle_messages().await {
                break;
            }

            if !self.subscribed.is_empty() {
                let mut values = vec![];

                for (id, request) in self.subscribed.iter() {
                    let value = self
                        .find_and_get_sensor_value(&request.hardware_indices, request.sensor_idx);

                    if let Some(value) = value {
                        values.push(SensorValue {
                            sensor_id: *id,
                            sensor_value: value,
                        });
                    }
                }

                if !values.is_empty() {
                    let _ = self
                        .sender_renderer
                        .send(RendererMessage::SensorValues(values))
                        .await;
                }
            }
        }
    }

    fn find_and_get_sensor_value(
        &self,
        hardware_indices: &[usize],
        sensor_idx: usize,
    ) -> Option<f32> {
        let hardware = self.computer.iter().nth(hardware_indices[0])?;
        let mut subhardware = None;

        for hardware_idx in hardware_indices.iter().skip(1) {
            subhardware = Some(hardware.subhardware_iter().nth(*hardware_idx)?);
        }

        let hardware = if subhardware.is_none() {
            hardware
        } else {
            subhardware?
        };

        let mut sensor = hardware.sensor_iter().nth(sensor_idx)?;

        sensor.get_value().ok()
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
                SensorMessage::RequestAllSensors => {
                    let mut sensors = vec![];

                    for (hardware_idx, hardware) in self.computer.iter().enumerate() {
                        for (sensor_idx, mut sensor) in hardware.sensor_iter().enumerate() {
                            let response = SensorsResponse {
                                sensor_name: sensor.get_name().unwrap_or_default(),
                                hardware_name: hardware.get_name().unwrap_or_default(),
                                hardware_indices: vec![hardware_idx],
                                sensor_type: sensor.get_type(),
                                sensor_idx,
                            };

                            sensors.push(response);
                        }

                        for (subhardware_idx, subhardware) in
                            hardware.subhardware_iter().enumerate()
                        {
                            for (subsensor_idx, mut sensor) in subhardware.sensor_iter().enumerate()
                            {
                                let response = SensorsResponse {
                                    sensor_name: sensor.get_name().unwrap_or_default(),
                                    hardware_name: subhardware.get_name().unwrap_or_default(),
                                    hardware_indices: vec![hardware_idx, subhardware_idx],
                                    sensor_type: sensor.get_type(),
                                    sensor_idx: subsensor_idx,
                                };

                                sensors.push(response);
                            }
                        }
                    }

                    let _ = self
                        .sender_from
                        .send(SensorMessage::AllSensorsResponse(sensors))
                        .await;
                }
                SensorMessage::SubscribeRequestList(requests) => {
                    self.subscribed = requests
                        .into_iter()
                        .map(|request| {
                            self.last_id += 1;
                            (self.last_id, request)
                        })
                        .collect();

                    let mut subscription_notifications: Vec<(&usize, SensorSubscriptionNotification)> = vec![];

                    for (id, request) in self.subscribed.iter() {
                        let hardware = self
                            .computer
                            .iter()
                            .nth(request.hardware_indices[0])
                            .unwrap();
                        let mut subhardware = None;

                        for hardware_idx in request.hardware_indices.iter().skip(1) {
                            subhardware = hardware.subhardware_iter().nth(*hardware_idx);
                        }

                        let hardware =
                            if subhardware.is_none() && request.hardware_indices.len() == 1 {
                                hardware
                            } else if subhardware.is_none() {
                                return true;
                            } else {
                                subhardware.unwrap()
                            };

                        let sensor = hardware.sensor_iter().nth(request.sensor_idx);

                        if let Some(mut sensor) = sensor {
                            let notification = SensorSubscriptionNotification {
                                sensor_id: *id,
                                sensor_name: sensor.get_name().unwrap_or_default(),
                                hardware_name: hardware.get_name().unwrap_or_default(),
                                sensor_type: sensor.get_type(),
                                sensor_value: sensor.get_value().unwrap_or_default(),
                            };

                            subscription_notifications.push((id, notification));
                        }
                    }
                }
                _ => {}
            }

            true
        } else {
            !matches!(received, Err(TryRecvError::Closed))
        }
    }
}
