pub trait DeviceCreator {
    async fn check_for_device() -> bool
    where
        Self: Sized;
    async fn create_device() -> anyhow::Result<Self>
    where
        Self: Sized;
    // async fn device_info(&self) -> DeviceInfo;
}
