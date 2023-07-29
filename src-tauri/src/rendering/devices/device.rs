use self::{capellix::Capellix, stream_deck_xl::StreamDeckXL, thermaltake::TTUltra};

#[path = "./capellix/device.rs"]
mod capellix;

#[path = "./thermaltake/device.rs"]
mod thermaltake;

#[path = "./streamdeck_xl/device.rs"]
mod stream_deck_xl;

pub struct DeviceContainer {
    device: Box<dyn Device>,
    device_info: DeviceInfo,
}

impl DeviceContainer {
    pub fn new() -> Result<Self, &'static str> {
        let _: Option<Capellix> = match Capellix::new() {
            Ok(device) => {
                return Ok(Self {
                    device: Box::new(device),
                    device_info: Capellix::device_info(),
                });
            }
            Err(_) => None,
        };

        let _: Option<TTUltra> = match TTUltra::new() {
            Ok(device) => {
                return Ok(Self {
                    device: Box::new(device),
                    device_info: TTUltra::device_info(),
                })
            }
            Err(_) => None,
        };

        let _: Option<StreamDeckXL> = match StreamDeckXL::new() {
            Ok(device) => {
                return Ok(Self {
                    device: Box::new(device),
                    device_info: StreamDeckXL::device_info(),
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
    pub fn send_image(&mut self, img: &[u8]) -> Result<(), &'static str> {
        self.device.send_image(img)
    }

    pub fn device_info(&self) -> DeviceInfo {
        self.device_info.clone()
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

#[derive(Clone, Debug)]
pub struct DeviceInfo {
    pub name: String,
    pub manufacturer: String,
    pub conflicting_processes: Vec<String>,
    pub width: u32,
    pub height: u32,
}

pub trait Device {
    fn init(&mut self) -> Result<(), &'static str>;
    fn close(&mut self) -> Result<(), &'static str>;
    fn reopen(&mut self) -> Result<(), &'static str>;
    fn send_image(&mut self, img: &[u8]) -> Result<(), &'static str>;
}
