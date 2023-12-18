use librehardwaremonitor_rs::SensorType;

pub enum RendererMessage {
    NewSubscribedSensors(Vec<SensorSubscriptionNotification>),
    SensorValues(Vec<SensorValue>),
    ReloadCurrentUrl(String),
    Shutdown,
}

pub enum SensorValueType {
    Min,
    Max,
    Current,
}

pub struct SensorValue {
    pub sensor_id: usize,
    pub sensor_value: f32,
}

pub struct SensorSubscriptionNotification {
    pub sensor_name: String,
    pub hardware_name: String,
    pub sensor_type: SensorType,
    pub sensor_id: usize,
    pub sensor_value: f32,
    // pub sensor_value_type: SensorValueType,
}
