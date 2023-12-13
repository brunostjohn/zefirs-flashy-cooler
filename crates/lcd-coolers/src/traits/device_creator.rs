pub trait DeviceCreator {
    async fn create_device() -> anyhow::Result<Self>
    where
        Self: Sized;
    // async fn device_info(&self) -> DeviceInfo;
}
