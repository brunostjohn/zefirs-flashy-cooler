use librehardwaremonitor_sys::SensorType as SensorTypeNative;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SensorType {
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

impl From<SensorTypeNative> for SensorType {
    fn from(sensor_type: SensorTypeNative) -> Self {
        match sensor_type {
            SensorTypeNative::Voltage => Self::Voltage,
            SensorTypeNative::Current => Self::Current,
            SensorTypeNative::Power => Self::Power,
            SensorTypeNative::Clock => Self::Clock,
            SensorTypeNative::Temperature => Self::Temperature,
            SensorTypeNative::Load => Self::Load,
            SensorTypeNative::Frequency => Self::Frequency,
            SensorTypeNative::Fan => Self::Fan,
            SensorTypeNative::Flow => Self::Flow,
            SensorTypeNative::Control => Self::Control,
            SensorTypeNative::Level => Self::Level,
            SensorTypeNative::Factor => Self::Factor,
            SensorTypeNative::Data => Self::Data,
            SensorTypeNative::SmallData => Self::SmallData,
            SensorTypeNative::Throughput => Self::Throughput,
            SensorTypeNative::TimeSpan => Self::TimeSpan,
            SensorTypeNative::Energy => Self::Energy,
            SensorTypeNative::Noise => Self::Noise,
        }
    }
}

impl From<SensorType> for SensorTypeNative {
    fn from(sensor_type: SensorType) -> Self {
        match sensor_type {
            SensorType::Voltage => Self::Voltage,
            SensorType::Current => Self::Current,
            SensorType::Power => Self::Power,
            SensorType::Clock => Self::Clock,
            SensorType::Temperature => Self::Temperature,
            SensorType::Load => Self::Load,
            SensorType::Frequency => Self::Frequency,
            SensorType::Fan => Self::Fan,
            SensorType::Flow => Self::Flow,
            SensorType::Control => Self::Control,
            SensorType::Level => Self::Level,
            SensorType::Factor => Self::Factor,
            SensorType::Data => Self::Data,
            SensorType::SmallData => Self::SmallData,
            SensorType::Throughput => Self::Throughput,
            SensorType::TimeSpan => Self::TimeSpan,
            SensorType::Energy => Self::Energy,
            SensorType::Noise => Self::Noise,
        }
    }
}
