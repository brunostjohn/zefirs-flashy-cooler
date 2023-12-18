use crate::{
    corsair::h150i::device::CorsairH150i,
    traits::{device_creator::DeviceCreator, display_cooler::DisplayCooler},
};

pub async fn find_and_create_device() -> anyhow::Result<impl DisplayCooler + DeviceCreator> {
    CorsairH150i::create_device().await
}
