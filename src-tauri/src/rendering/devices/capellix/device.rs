use std::{
    thread,
    time::{Duration, SystemTime},
};

use hidapi::{HidApi, HidDevice};
use image::RgbImage;

use self::constants::constants::*;

use super::{Device, DeviceCreator};
mod constants;

pub struct Capellix {
    pub device: Option<HidDevice>,
    api: HidApi,
    init_time: SystemTime,
    unfucks_sent: u16,
}

impl DeviceCreator for Capellix {
    fn new() -> Result<Capellix, &'static str> {
        let api = match HidApi::new() {
            Ok(api) => api,
            Err(_) => panic!("Failed to initialise HidApi!"),
        };

        let mut device_handle: Option<HidDevice> = None;

        for device in api.device_list() {
            if device.vendor_id() == VENDOR_ID {
                if device.product_id() == PRODUCT_ID || device.product_id() == PRODUCT_ID_V2 {
                    device_handle = Some(match device.open_device(&api) {
                        Ok(device) => device,
                        Err(_) => return Err("Failed to open device!"),
                    });
                }
            }
        }

        if device_handle.is_none() {
            return Err("Failed to find device!");
        } else {
            return Ok(Capellix {
                api: api,
                device: Some(device_handle.unwrap()),
                init_time: SystemTime::now(),
                unfucks_sent: 0,
            });
        }
    }
}

impl Device for Capellix {
    fn init(&mut self) -> Result<(), &'static str> {
        self.init_time = SystemTime::now();
        let handle = match &self.device {
            Some(device) => device,
            None => return Err("Device can only be initialised once."),
        };
        match handle.send_feature_report(&[
            CONTROL_REQUEST,
            DEVICE_STAT,
            0x01,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ]) {
            Err(_) => return Err("Failed to initialise device! Request: DEVICE_STAT."),
            _ => {}
        };

        match handle.send_feature_report(&[
            CONTROL_REQUEST,
            DEVICE_ALIVE,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ]) {
            Err(_) => return Err("Failed to initialise device! Request: DEVICE_ALIVE."),
            _ => {}
        };

        match handle.send_feature_report(&[
            CONTROL_REQUEST,
            0x20,
            0x00,
            0x19,
            0x79,
            0xe7,
            0x32,
            0x2e,
        ]) {
            Err(_) => return Err("Failed to initialise device! Request: 0x20."),
            _ => {}
        };

        match handle.send_feature_report(&[
            CONTROL_REQUEST,
            SET_INTERFACE,
            0x40,
            0x01,
            0x79,
            0xe7,
            0x32,
            0x2e,
        ]) {
            Err(_) => return Err("Failed to initialise device! Request: SET_INTERFACE."),
            _ => {}
        };

        std::thread::sleep(std::time::Duration::from_millis(5));

        Ok(())
    }

    fn close(&mut self) -> Result<(), &'static str> {
        thread::sleep(Duration::from_millis(5));

        let handle = match &self.device {
            Some(device) => device,
            None => return Err("Device already closed."),
        };

        match handle.send_feature_report(&[
            CONTROL_REQUEST,
            0x1e,
            0x40,
            0x01,
            0x43,
            0x00,
            0x69,
            0x00,
        ]) {
            Err(_) => return Err("Failed to close device."),
            _ => {}
        }

        self.device = None;

        Ok(())
    }

    fn reopen(&mut self) -> Result<(), &'static str> {
        let mut device_handle: Option<HidDevice> = None;

        for device in self.api.device_list() {
            if device.vendor_id() == VENDOR_ID {
                if device.product_id() == PRODUCT_ID || device.product_id() == PRODUCT_ID_V2 {
                    device_handle = Some(match device.open_device(&self.api) {
                        Ok(device) => device,
                        Err(_) => return Err("Failed to open device!"),
                    });
                }
            }
        }

        if device_handle.is_none() {
            return Err("Failed to find device!");
        } else {
            self.device = device_handle;
            Ok(())
        }
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
                0x05,
                0x40,
                0x00,
                packets_sent,
                0x00,
                (chunk_length & 0xff) as u8,
                (chunk_length >> 8 & 0xff) as u8,
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

            if chunk_length < 1016 {
                let time_passed = match SystemTime::now().duration_since(self.init_time) {
                    Ok(val) => val,
                    _ => Duration::from_millis(1),
                };

                if (time_passed.as_secs() / 30) > self.unfucks_sent as u64 {
                    let unfuck_packet = [
                        CONTROL_REQUEST,
                        DEVICE_ALIVE,
                        0x40,
                        0x01,
                        packets_sent,
                        0x00,
                        (chunk_length & 0xff) as u8,
                        (chunk_length >> 8 & 0xff) as u8,
                    ];

                    self.unfucks_sent += 1;

                    if self.unfucks_sent == u16::MAX {
                        self.unfucks_sent = 0;
                        self.init_time = SystemTime::now();
                    }

                    match handle.send_feature_report(&unfuck_packet) {
                        Err(_) => return Err("Failed to send unfuck packet!"),
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }
}
