use librehardwaremonitor_rs::SensorType;
use serde::Serialize;

pub enum RendererMessage {
    NewSubscribedSensors(Vec<SensorSubscriptionNotification>),
    SensorValues(Vec<SensorValue>),
    ReloadCurrentUrl(String),
    Shutdown,
}

// pub enum SensorValueType {
//     Min,
//     Max,
//     Current,
// }

pub struct SensorValue {
    pub sensor_id: usize,
    pub sensor_value: f32,
}

#[derive(Serialize)]
#[serde(remote = "SensorType")]
pub enum SensorTypeLocal {
    Voltage,
    Current,
    Power,
    Clock,
    Temperature,
    Load,
    Frequency,
    Fan,
    Flow,
    Control,
    Level,
    Factor,
    Data,
    SmallData,
    Throughput,
    TimeSpan,
    Energy,
    Noise,
}

#[derive(Serialize)]
pub struct SensorSubscriptionNotification {
    pub sensor_name: String,
    pub hardware_name: String,
    #[serde(with = "SensorTypeLocal")]
    pub sensor_type: SensorType,
    pub sensor_id: usize,
    pub sensor_value: f32,
    #[serde(skip)]
    pub code_name: String,
    // pub sensor_value_type: SensorValueType,
}
