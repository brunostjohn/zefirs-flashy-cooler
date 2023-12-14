use tokio::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SensorMessage {
    ChangePoll(Duration),
    Shutdown,
}
