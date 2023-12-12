use crate::traits::device_creator::DeviceCreator;
use anyhow::Context;
use windows::Devices::{Enumeration::DeviceInformation, HumanInterfaceDevice::HidDevice};

use super::constants::{PRODUCT_ID, VENDOR_ID};

pub struct CorsairH150i {}

impl DeviceCreator for CorsairH150i {
    async fn check_for_device() -> bool
    where
        Self: Sized,
    {
        todo!();
    }

    async fn create_device() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let selector = HidDevice::GetDeviceSelectorVidPid(0xFFC0, 0x1, VENDOR_ID, PRODUCT_ID)
            .context("Failed to get device selector")?;
        let devices = DeviceInformation::FindAllAsyncAqsFilter(&selector)
            .context("Failed to find devices")?
            .await
            .context("Failed to find devices")?;

        Ok(CorsairH150i {})
    }
}
