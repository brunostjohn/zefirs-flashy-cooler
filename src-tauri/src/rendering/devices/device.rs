use image::RgbImage;

use self::{capellix::Capellix, thermaltake::TTUltra};

#[path = "./capellix/device.rs"]
mod capellix;

#[path = "./thermaltake/device.rs"]
mod thermaltake;

pub struct DeviceContainer {
    device: Box<dyn Device>,
}

impl DeviceContainer {
    pub fn new() -> Result<Self, &'static str> {
        let _: Option<TTUltra> = match TTUltra::new() {
            Ok(device) => {
                return Ok(Self {
                    device: Box::new(device),
                })
            }
            Err(_) => None,
        };

        let _: Option<Capellix> = match Capellix::new() {
            Ok(device) => {
                return Ok(Self {
                    device: Box::new(device),
                });
            }
            Err(_) => None,
        };

        Err("Failed to open any device!")
    }

    #[inline(always)]
    pub fn init(&mut self) -> Result<(), &'static str> {
        self.device.init()
    }

    #[inline(always)]
    pub fn close(&mut self) -> Result<(), &'static str> {
        self.device.close()
    }

    #[inline(always)]
    pub fn reopen(&mut self) -> Result<(), &'static str> {
        self.device.reopen()
    }

    #[inline(always)]
    pub fn send_image(&mut self, img: &RgbImage) -> Result<(), &'static str> {
        self.device.send_image(img)
    }
}

pub trait DeviceCreator {
    fn new() -> Result<Self, &'static str>
    where
        Self: Sized;
    fn device_info() -> DeviceInfo
    where
        Self: Sized;
}

pub struct DeviceInfo {
    pub name: String,
    pub manufacturer: String,
    pub conflicting_processes: Vec<String>,
}

pub trait Device {
    fn init(&mut self) -> Result<(), &'static str>;
    fn close(&mut self) -> Result<(), &'static str>;
    fn reopen(&mut self) -> Result<(), &'static str>;
    fn send_image(&mut self, img: &RgbImage) -> Result<(), &'static str>;
}
