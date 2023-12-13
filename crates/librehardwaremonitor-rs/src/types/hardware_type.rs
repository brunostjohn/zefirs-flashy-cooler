use librehardwaremonitor_sys::HardwareType as HardwareTypeNative;

pub enum HardwareType {
    Motherboard,
    SuperIO,
    Cpu,
    Memory,
    GpuNvidia,
    GpuAmd,
    GpuIntel,
    Storage,
    Network,
    Cooler,
    EmbeddedController,
    Psu,
    Battery,
}

impl From<HardwareTypeNative> for HardwareType {
    fn from(hardware_type: HardwareTypeNative) -> Self {
        match hardware_type {
            HardwareTypeNative::Motherboard => Self::Motherboard,
            HardwareTypeNative::SuperIO => Self::SuperIO,
            HardwareTypeNative::Cpu => Self::Cpu,
            HardwareTypeNative::Memory => Self::Memory,
            HardwareTypeNative::GpuNvidia => Self::GpuNvidia,
            HardwareTypeNative::GpuAmd => Self::GpuAmd,
            HardwareTypeNative::GpuIntel => Self::GpuIntel,
            HardwareTypeNative::Storage => Self::Storage,
            HardwareTypeNative::Network => Self::Network,
            HardwareTypeNative::Cooler => Self::Cooler,
            HardwareTypeNative::EmbeddedController => Self::EmbeddedController,
            HardwareTypeNative::Psu => Self::Psu,
            HardwareTypeNative::Battery => Self::Battery,
        }
    }
}

impl From<HardwareType> for HardwareTypeNative {
    fn from(hardware_type: HardwareType) -> Self {
        match hardware_type {
            HardwareType::Motherboard => Self::Motherboard,
            HardwareType::SuperIO => Self::SuperIO,
            HardwareType::Cpu => Self::Cpu,
            HardwareType::Memory => Self::Memory,
            HardwareType::GpuNvidia => Self::GpuNvidia,
            HardwareType::GpuAmd => Self::GpuAmd,
            HardwareType::GpuIntel => Self::GpuIntel,
            HardwareType::Storage => Self::Storage,
            HardwareType::Network => Self::Network,
            HardwareType::Cooler => Self::Cooler,
            HardwareType::EmbeddedController => Self::EmbeddedController,
            HardwareType::Psu => Self::Psu,
            HardwareType::Battery => Self::Battery,
        }
    }
}
