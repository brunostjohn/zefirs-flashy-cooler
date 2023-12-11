use crate::types::log_level;
use ultralight_sys::ULLogLevel;

static mut CALLBACK: Option<Box<dyn Fn(log_level::ULLogLevel, &str)>> = None;

pub(crate) unsafe extern "C" fn logger_wrapper(
    log_level: ULLogLevel,
    message: ultralight_sys::ULString,
) {
    let message = crate::string::ULString::from_raw(message);
    let message = message.as_str();
    if let Some(ref mut callback) = CALLBACK {
        callback(log_level.into(), message);
    }
}

pub(crate) fn set_logger<F>(logger: F)
where
    F: Fn(crate::types::log_level::ULLogLevel, &str) + 'static,
{
    unsafe {
        CALLBACK = Some(Box::new(logger));
    }
}

pub(crate) fn default_logger(log_level: crate::types::log_level::ULLogLevel, message: &str) {
    println!("{:?}: {}", log_level, message);
}
