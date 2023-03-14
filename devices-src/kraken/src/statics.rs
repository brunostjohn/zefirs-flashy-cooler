pub mod statics {
    pub const WRITE_LENGTH: usize = 64;
    pub const BULK_WRITE_LENGTH: usize = 512;
    pub const BULK_ENDPOINT: u8 = 0x02;
    pub const TEN_MS: std::time::Duration = std::time::Duration::from_millis(10);
    pub const BULK_INTERFACE: u8 = 0x00;
}
