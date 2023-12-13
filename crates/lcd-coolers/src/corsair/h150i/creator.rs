use super::{
    constants::{PRODUCT_ID, PRODUCT_ID_V2, VENDOR_ID},
    counter::Counter,
    device::CorsairH150i,
};
use crate::traits::device_creator::DeviceCreator;
use anyhow::Context;
use hidapi::HidApi;
use tokio::task;
use turbojpeg::{Compressor, Image, OutputBuf, PixelFormat, Subsamp};

impl<'a> DeviceCreator for CorsairH150i<'a> {
    async fn create_device() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let (api, device) = task::spawn_blocking(|| -> anyhow::Result<_> {
            let api = HidApi::new().context("Failed to get HID API")?;
            let device = api
                .open(VENDOR_ID, PRODUCT_ID)
                .or_else(|_| api.open(VENDOR_ID, PRODUCT_ID_V2))
                .context("Failed to open device")?;

            Ok((api, device))
        })
        .await??;

        let mut compressor = Compressor::new().context("Failed to create compressor")?;
        compressor.set_quality(95);
        compressor.set_subsamp(Subsamp::Sub2x2);

        Ok(Self {
            api,
            device,
            image_frame: Image {
                pixels: &[],
                width: 480,
                height: 480,
                pitch: 480 * 4,
                format: PixelFormat::RGBA,
            },
            compression_buffer: OutputBuf::allocate_owned(
                compressor
                    .buf_len(480, 480)
                    .context("Failed to get buffer length")?,
            ),
            compressor,
            packet: Vec::with_capacity(1024),
            unfuck_counter: Counter::new(1000 * 29),
        })
    }
}
