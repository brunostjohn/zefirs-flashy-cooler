#![allow(unused_mut)]

use std::{path::Path, ptr};

use self::logger::{default_logger, logger_wrapper, set_logger};
use ultralight_sys::ULLogger;

mod gpu;
mod logger;

pub struct ULPlatformBuilder {
    is_file_system_set: bool,
    is_font_loader_set: bool,
}

impl Default for ULPlatformBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ULPlatformBuilder {
    pub fn new() -> Self {
        ultralight_sys::check_for_resources();

        let me = ULPlatformBuilder {
            is_file_system_set: false,
            is_font_loader_set: false,
        };

        unsafe {
            ultralight_sys::ulPlatformSetLogger(ULLogger {
                log_message: Some(logger_wrapper),
            })
        };

        set_logger(default_logger);

        me
    }

    pub fn enable_file_logger<S: AsRef<Path>>(mut self, log_path: S) -> Self {
        let log_path = crate::string::ULString::new(log_path.as_ref().to_str().unwrap());
        unsafe { ultralight_sys::ulEnableDefaultLogger(*log_path) };
        self
    }

    pub fn set_logger<F>(mut self, logger: F) -> Self
    where
        F: Fn(crate::types::log_level::ULLogLevel, &str) + 'static,
    {
        logger::set_logger(logger);
        self
    }

    pub fn enable_platform_file_system(mut self) -> Self {
        if self.is_file_system_set {
            panic!("File system handler already set!");
        }
        unsafe { ultralight_sys::ulEnablePlatformFileSystem(ptr::null_mut()) };
        self.is_file_system_set = true;
        self
    }

    pub fn enable_platform_file_system_with_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        if self.is_file_system_set {
            panic!("File system handler already set!");
        }
        let path = crate::string::ULString::new(path.as_ref().to_str().unwrap());
        unsafe { ultralight_sys::ulEnablePlatformFileSystem(*path) };
        self.is_file_system_set = true;
        self
    }

    pub fn enable_platform_font_loader(mut self) -> Self {
        unsafe { ultralight_sys::ulEnablePlatformFontLoader() };
        self.is_font_loader_set = true;
        self
    }

    pub fn enable_gpu_driver(mut self) -> Self {
        todo!("GPU driver not implemented yet");
    }

    pub fn build(self) {
        if !self.is_file_system_set {
            panic!("File system handler not set!");
        }

        if !self.is_font_loader_set {
            panic!("Font loader not set!");
        }
    }
}

#[cfg(test)]
mod tests;
