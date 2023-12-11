#[derive(Debug)]
pub enum ULLogLevel {
    Error,
    Warning,
    Info,
}

impl From<ULLogLevel> for ultralight_sys::ULLogLevel {
    fn from(log_level: ULLogLevel) -> Self {
        match log_level {
            ULLogLevel::Error => ultralight_sys::ULLogLevel_kLogLevel_Error,
            ULLogLevel::Warning => ultralight_sys::ULLogLevel_kLogLevel_Warning,
            ULLogLevel::Info => ultralight_sys::ULLogLevel_kLogLevel_Info,
        }
    }
}

impl From<ultralight_sys::ULLogLevel> for ULLogLevel {
    fn from(log_level: ultralight_sys::ULLogLevel) -> Self {
        match log_level {
            ultralight_sys::ULLogLevel_kLogLevel_Error => ULLogLevel::Error,
            ultralight_sys::ULLogLevel_kLogLevel_Warning => ULLogLevel::Warning,
            ultralight_sys::ULLogLevel_kLogLevel_Info => ULLogLevel::Info,
            _ => panic!("Unknown log level"),
        }
    }
}
