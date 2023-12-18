use crate::DeviceInfo;

#[allow(async_fn_in_trait)]
pub trait DeviceCreator {
    async fn create_device() -> anyhow::Result<Self>
    where
        Self: Sized;
    async fn device_info(&self) -> DeviceInfo;
}
