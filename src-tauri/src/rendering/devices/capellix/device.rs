use std::{thread, time::Duration};

use hidapi::{HidApi, HidDevice};
use turbojpeg::OutputBuf;

use crate::rendering::helpers_threading::EventTicker;

use self::constants::constants::*;

use super::{Device, DeviceCreator};
mod constants;

pub struct Capellix<'a> {
    pub device: Option<HidDevice>,
    api: HidApi,
    unfuck_counter: EventTicker,
    compressor: turbojpeg::Compressor,
    image_frame: turbojpeg::Image<&'a [u8]>,
    compression_buffer: OutputBuf<'a>,
    packet: Vec<u8>,
}

impl<'a> DeviceCreator for Capellix<'a> {
    fn new() -> Result<Capellix<'a>, &'static str> {
        let api = match HidApi::new() {
            Ok(api) => api,
            Err(_) => panic!("Failed to initialise HidApi!"),
        };

        let mut device_handle: Option<HidDevice> = None;

        for device in api.device_list() {
            if device.vendor_id() == VENDOR_ID
                && (device.product_id() == PRODUCT_ID || device.product_id() == PRODUCT_ID_V2)
            {
                device_handle = Some(match device.open_device(&api) {
                    Ok(device) => device,
                    Err(_) => return Err("Failed to open device!"),
                });
            }
        }

        println!("I exist bruh");

        if let Some(device_handle) = device_handle {
            let mut compressor = turbojpeg::Compressor::new().or(Err("Failed to cook compress"))?;
            compressor.set_quality(95);
            compressor.set_subsamp(turbojpeg::Subsamp::Sub2x2);

            println!("Returning myself");
            Ok(Capellix {
                api,
                device: Some(device_handle),
                unfuck_counter: EventTicker::new(1000 * 29),
                compression_buffer: OutputBuf::allocate_owned(
                    compressor.buf_len(480, 480).unwrap(),
                ),
                compressor,
                image_frame: turbojpeg::Image {
                    pixels: &[],
                    width: 480,
                    pitch: 480 * 4,
                    height: 480,
                    format: turbojpeg::PixelFormat::RGBA,
                },
                packet: Vec::with_capacity(1024),
            })
        } else {
            Err("Failed to find device!")
        }
    }
    fn device_info() -> super::DeviceInfo
    where
        Self: Sized,
    {
        super::DeviceInfo {
            name: "Corsair iCUE Capellix LCD Cooler".to_string(),
            manufacturer: "Corsair".to_string(),
            conflicting_processes: vec!["iCUE.exe".to_string()],
        }
    }
}

impl<'a> Device for Capellix<'a> {
    fn init(&mut self) -> Result<(), &'static str> {
        self.unfuck_counter = EventTicker::new(1000 * 29);

        println!("initing self!");

        let handle = match &self.device {
            Some(device) => device,
            None => return Err("Device can only be initialised once."),
        };
        if handle
            .send_feature_report(&[
                CONTROL_REQUEST,
                DEVICE_STAT,
                0x01,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
            ])
            .is_err()
        {
            return Err("Failed to initialise device! Request: DEVICE_STAT.");
        };

        if handle
            .send_feature_report(&[
                CONTROL_REQUEST,
                DEVICE_ALIVE,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
                0x00,
            ])
            .is_err()
        {
            return Err("Failed to initialise device! Request: DEVICE_ALIVE.");
        };

        if handle
            .send_feature_report(&[CONTROL_REQUEST, 0x20, 0x00, 0x19, 0x79, 0xe7, 0x32, 0x2e])
            .is_err()
        {
            return Err("Failed to initialise device! Request: 0x20.");
        };

        if handle
            .send_feature_report(&[
                CONTROL_REQUEST,
                SET_INTERFACE,
                0x40,
                0x01,
                0x79,
                0xe7,
                0x32,
                0x2e,
            ])
            .is_err()
        {
            return Err("Failed to initialise device! Request: SET_INTERFACE.");
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

        if handle
            .send_feature_report(&[CONTROL_REQUEST, 0x1e, 0x40, 0x01, 0x43, 0x00, 0x69, 0x00])
            .is_err()
        {
            return Err("Failed to close device.");
        }

        self.device = None;

        Ok(())
    }

    fn reopen(&mut self) -> Result<(), &'static str> {
        let mut device_handle: Option<HidDevice> = None;

        for device in self.api.device_list() {
            if device.vendor_id() == VENDOR_ID
                && (device.product_id() == PRODUCT_ID || device.product_id() == PRODUCT_ID_V2)
            {
                device_handle = Some(match device.open_device(&self.api) {
                    Ok(device) => device,
                    Err(_) => return Err("Failed to open device!"),
                });
            }
        }

        if device_handle.is_none() {
            Err("Failed to find device!")
        } else {
            self.device = device_handle;
            Ok(())
        }
    }

    fn send_image(&mut self, img: &[u8]) -> Result<(), &'static str> {
        let mut last_image = [0u8; 1016].as_slice();

        let mut image_struct = self.image_frame;
        image_struct.pixels = img;

        self.compressor
            .compress(image_struct, &mut self.compression_buffer)
            .unwrap();

        if self.compression_buffer.is_empty() {
            self.compressor = turbojpeg::Compressor::new().unwrap();
            self.compressor.set_quality(95);
            self.compressor.set_subsamp(turbojpeg::Subsamp::Sub2x2);
        }

        let handle = match &self.device {
            Some(device) => device,
            None => return Err("No device initialised!"),
        };

        for (packets_sent, chunk) in self.compression_buffer.chunks(1016).enumerate() {
            let chunk_length = chunk.len() as u16;

            self.packet.extend([IMG_TX, 0x05, 0x40].iter());

            if chunk_length == 1016 {
                self.packet
                    .extend([0x00, packets_sent.try_into().unwrap(), 0x00, 0xf8, 0x03].iter());
            } else {
                self.packet.extend(
                    [
                        0x01,
                        packets_sent.try_into().unwrap(),
                        0x00,
                        (chunk_length & 0xff) as u8,
                        (chunk_length >> 8 & 0xff) as u8,
                    ]
                    .iter(),
                );
            }

            self.packet.extend(chunk.iter());

            if chunk_length < 1016 {
                self.packet
                    .extend(&mut last_image[chunk_length as usize..].iter());
            }

            last_image = chunk;

            if handle.write(&self.packet).is_err() {
                return Err("Failed to write to device!");
            }

            if chunk_length < 1016 && self.unfuck_counter.check_time() {
                let unfuck_packet = [
                    CONTROL_REQUEST,
                    DEVICE_ALIVE,
                    0x40,
                    0x01,
                    packets_sent.try_into().unwrap(),
                    0x00,
                    (chunk_length & 0xff) as u8,
                    (chunk_length >> 8 & 0xff) as u8,
                ];

                if handle.send_feature_report(&unfuck_packet).is_err() {
                    return Err("Failed to send unfuck packet!");
                }
            }

            self.packet.clear();
        }

        Ok(())
    }
}
