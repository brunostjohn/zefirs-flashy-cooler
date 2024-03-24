use librehardwaremonitor_rs::SensorType;
use tokio::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SensorMessage {
    ChangePoll(Duration),
    RequestAllSensors,
    AllSensorsResponse(Vec<SensorsResponse>),
    SubscribeRequestList(Vec<SubscribeRequest>),
    FindAndSubscribeByIdWithCodeName(Vec<(String, String)>),
    Shutdown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SensorsResponse {
    pub sensor_name: String,
    pub hardware_name: String,
    pub hardware_indices: Vec<usize>,
    pub sensor_type: SensorType,
    pub sensor_idx: usize,
    pub sensor_persist_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubscribeRequest {
    pub hardware_indices: Vec<usize>,
    pub name_as: String,
    pub sensor_idx: usize,
}
