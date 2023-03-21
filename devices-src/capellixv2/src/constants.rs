pub mod constants {
    pub const VENDOR_ID: u16 = 0x1b1c;
    pub const PRODUCT_ID: u16 = 0x0c33;
    pub const HID_INTERFACE: u8 = 0x0;
    pub const OUT_ENDPOINT: u8 = 0x09;
    pub const TIMEOUT: std::time::Duration = std::time::Duration::from_millis(5);
    pub const CONTROL_REQUEST: u8 = 0x03;
    pub const DEVICE_STAT: u8 = 0x1d;
    pub const BEGIN_ACCEPTING: u16 = 0x0001;
    pub const DEVICE_ALIVE: u8 = 0x19;
    pub const SET_INTERFACE: u8 = 0x0b;
}
