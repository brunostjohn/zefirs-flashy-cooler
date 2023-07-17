use self::constants::constants::*;
use super::{Device, DeviceCreator};
mod constants;

use hidapi::{HidApi, HidDevice};

pub struct TTUltra {
    pub device: Option<HidDevice>,
    api: HidApi,
}

impl DeviceCreator for TTUltra {
    fn new() -> Result<TTUltra, &'static str> {
        let api = HidApi::new().or(Err("Failed to open device"))?;
        let device = api
            .open(VENDOR_ID, PRODUCT_ID)
            .or(Err("Failed to open device!"))?;

        Ok(TTUltra {
            device: Some(device),
            api,
        })
    }
    fn device_info() -> super::DeviceInfo
    where
        Self: Sized,
    {
        super::DeviceInfo {
            name: "Thermaltake Toughliquid LCD Cooler".to_string(),
            manufacturer: "Thermaltake".to_string(),
            conflicting_processes: vec![],
        }
    }
}

impl Device for TTUltra {
    fn init(&mut self) -> Result<(), &'static str> {
        Ok(())
    }

    fn reopen(&mut self) -> Result<(), &'static str> {
        let device_handle = self
            .api
            .open(VENDOR_ID, PRODUCT_ID)
            .or(Err("Failed to reopen device!"))?;

        self.device = Some(device_handle);

        Ok(())
    }

    fn close(&mut self) -> Result<(), &'static str> {
        Ok(())
    }

    fn send_image(&mut self, img: &[u8]) -> Result<(), &'static str> {
        let mut last_image = [0u8; 1016].as_slice();

        let mut compressor = turbojpeg::Compressor::new().unwrap();
        compressor.set_quality(95);
        compressor.set_subsamp(turbojpeg::Subsamp::Sub2x2);

        let image_struct = turbojpeg::Image {
            pixels: img,
            width: 480,
            pitch: 480 * 4,
            height: 480,
            format: turbojpeg::PixelFormat::RGBA,
        };

        let image = compressor.compress_to_owned(image_struct).unwrap();

        let handle = match &self.device {
            Some(device) => device,
            None => return Err("No device initialised!"),
        };

        let mut packet = Vec::with_capacity(1028);

        for (packets_sent, chunk) in image.chunks(1016).enumerate() {
            let chunk_length = chunk.len() as u16;

            packet.extend([IMG_TX, 0x09, 0x65].iter());

            if chunk_length == 1016 {
                packet.extend([0x00, 0xf8, 0x03].iter());
            } else {
                packet.extend(
                    [
                        0x01,
                        (chunk_length & 0xff) as u8,
                        (chunk_length >> 8 & 0xff) as u8,
                    ]
                    .iter(),
                );
            }

            packet.extend([packets_sent.try_into().unwrap(), 0x00].iter());

            packet.extend(chunk.iter());

            if chunk_length < 1016 {
                packet.extend(&mut last_image[chunk_length as usize..].iter());
            }

            last_image = chunk;

            if handle.write(&packet).is_err() {
                return Err("Failed to write to device!");
            };
        }
        packet.clear();

        Ok(())
    }
}
