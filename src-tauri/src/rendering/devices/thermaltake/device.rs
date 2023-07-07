use self::constants::constants::*;
use super::{Device, DeviceCreator};
mod constants;

use hidapi::{HidApi, HidDevice};
use image::RgbImage;

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

    fn send_image(&mut self, img: &RgbImage) -> Result<(), &'static str> {
        let mut packets_sent = 0;
        let mut last_image: Vec<u8> = vec![];
        let image = turbojpeg::compress_image(img, 95, turbojpeg::Subsamp::Sub2x2)
            .unwrap()
            .to_vec();

        let handle = match &self.device {
            Some(device) => device,
            None => return Err("No device initialised!"),
        };

        for chunk in image.chunks(1016) {
            let chunk_length = chunk.len() as u16;

            let mut packet = vec![
                IMG_TX,
                0x09,
                0x65,
                0x00,
                (chunk_length & 0xff) as u8,
                (chunk_length >> 8 & 0xff) as u8,
                packets_sent,
                0x00,
            ];

            packet.extend(chunk.iter());

            if chunk_length < 1016 {
                packet[3] = 0x01;
                packet.extend(&mut last_image[chunk_length as usize..].iter());
            }

            last_image = chunk.to_vec();

            match handle.write(&packet) {
                Err(_) => return Err("Failed to write to device!"),
                _ => {}
            };
            packets_sent += 1;
        }

        Ok(())
    }
}
