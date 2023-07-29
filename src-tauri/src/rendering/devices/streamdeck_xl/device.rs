use hidapi::{HidApi, HidDevice};
use image::DynamicImage;
use turbojpeg::OutputBuf;

use super::{Device, DeviceCreator};

const GAP: u32 = 20;

pub struct StreamDeckXL<'a> {
    pub device: Option<HidDevice>,
    api: HidApi,
    compressor: turbojpeg::Compressor,
    image_frame: turbojpeg::Image<&'a [u8]>,
    compression_buffer: OutputBuf<'a>,
    packet: Vec<u8>,
}

impl<'a> DeviceCreator for StreamDeckXL<'a> {
    fn new() -> Result<StreamDeckXL<'a>, &'static str> {
        let api = match HidApi::new() {
            Ok(api) => api,
            Err(_) => panic!("Failed to initialise HidApi!"),
        };

        let device_handle: Option<HidDevice> =
            Some(api.open(0x0fd9, 0x006c).or(Err("Failed to open device"))?);
        println!("I exist bruh");

        if let Some(device_handle) = device_handle {
            let mut compressor = turbojpeg::Compressor::new().or(Err("Failed to cook compress"))?;
            compressor.set_quality(95);
            compressor.set_subsamp(turbojpeg::Subsamp::Sub2x2);

            println!("Returning myself");
            Ok(StreamDeckXL {
                api,
                device: Some(device_handle),
                compression_buffer: OutputBuf::allocate_owned(
                    compressor.buf_len(480, 480).unwrap(),
                ),
                compressor,
                image_frame: turbojpeg::Image {
                    pixels: &[],
                    width: 96,
                    pitch: 96 * 4,
                    height: 96,
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
            name: "Stream Deck XL".to_string(),
            manufacturer: "Corsair".to_string(),
            conflicting_processes: vec!["iCUE.exe".to_string()],
            width: (768 + 7 * GAP),
            height: (384 + 3 * GAP),
        }
    }
}

impl<'a> Device for StreamDeckXL<'a> {
    fn init(&mut self) -> Result<(), &'static str> {
        Ok(())
    }

    fn close(&mut self) -> Result<(), &'static str> {
        Ok(())
    }

    fn reopen(&mut self) -> Result<(), &'static str> {
        let device_handle: Option<HidDevice> = self.api.open(0x0fd9, 0x006c).ok();

        if device_handle.is_none() {
            Err("Failed to find device!")
        } else {
            self.device = device_handle;
            Ok(())
        }
    }

    fn send_image(&mut self, img: &[u8]) -> Result<(), &'static str> {
        let handle = match &self.device {
            Some(device) => device,
            None => return Err("No device initialised!"),
        };

        let rgba = image::RgbaImage::from_raw(96 * 8 + 7 * GAP, 96 * 4 + 3 * GAP, img.to_vec())
            .ok_or("Failed to create image")?;

        let dynamic = DynamicImage::from(rgba);

        for row in 0..4 {
            for button in 0..8 {
                let image =
                    dynamic.crop_imm(button * 96 + GAP * button, row * 96 + GAP * row, 96, 96);
                let image = image.rotate180();

                let inner = image.as_bytes();

                if inner.len() == 96 * 96 * 4 {
                    let mut image_struct = self.image_frame;
                    image_struct.pixels = inner;

                    self.compressor
                        .compress(image_struct, &mut self.compression_buffer)
                        .unwrap();

                    if self.compression_buffer.is_empty() {
                        self.compressor = turbojpeg::Compressor::new().unwrap();
                        self.compressor.set_quality(30);
                        self.compressor.set_subsamp(turbojpeg::Subsamp::Sub2x2);
                    }

                    let mut last_image = [0u8; 1016].as_slice();

                    for (packets_sent, chunk) in self.compression_buffer.chunks(1016).enumerate() {
                        let chunk_length = chunk.len() as u16;

                        self.packet
                            .extend([0x02, 0x07, (button + (row * 8)) as u8].iter());

                        if chunk_length == 1016 {
                            self.packet.extend(
                                [0x00, 0xf8, 0x03, packets_sent.try_into().unwrap(), 0x00].iter(),
                            );
                        } else {
                            self.packet.extend(
                                [
                                    0x01,
                                    (chunk_length & 0xff) as u8,
                                    (chunk_length >> 8 & 0xff) as u8,
                                    packets_sent.try_into().unwrap(),
                                    0x00,
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

                        self.packet.clear();
                    }
                }

                self.packet.clear();
            }
        }
        Ok(())
    }
}
