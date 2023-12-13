use thiserror::Error;

#[derive(Debug, Error)]
pub enum LibreError {
    #[error("The name provided is invalid.")]
    InvalidName,
    #[error("Failed to set the provided name.")]
    FailedToSetName,
    #[error("Failed to get the value of the sensor.")]
    FailedToGetSensorValue
}
