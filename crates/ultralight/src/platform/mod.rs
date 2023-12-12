#![allow(unused_mut)]

//! A builder for creating a platform context.
//!
//! This MUST be called before creating any other Ultralight objects.
//!
//! # Example
//! ```rust
//! use ultralight::{platform::ULPlatformBuilder, renderer::ULRendererBuilder};
//!
//! let platform = ULPlatformBuilder::new()
//!    .enable_platform_file_system()
//!    .enable_platform_font_loader()
//!    .build();
//!
//! let renderer = ULRendererBuilder::new()
//!    .build();
//! ```

use std::{path::Path, ptr};

use crate::ULLogLevel;

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
    /// Creates a new platform builder.
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

    /// Enables the default logger.
    ///
    /// The default logger will log to the given path. The path must exist. If the path does not exist, this will panic as Ultralight can react badly to a non-existent log path.
    pub fn enable_file_logger<S: AsRef<Path>>(mut self, log_path: S) -> Self {
        let log_path = log_path.as_ref();
        // assert!(log_path.exists(), "Log path does not exist!");
        let log_path = crate::string::ULString::new(log_path.to_str().unwrap());
        unsafe { ultralight_sys::ulEnableDefaultLogger(*log_path) };
        self
    }

    /// Sets a custom logger.
    ///
    /// The logger will be called with the log level and the message. It must have a `'static` lifetime.
    ///
    /// # Example
    /// ```rust
    /// use ultralight::{ULLogLevel, platform::ULPlatformBuilder};
    ///
    /// let platform = ULPlatformBuilder::new()
    ///        .set_logger(|level, message| {
    ///             println!("{}: {}", level, message);
    ///         })
    ///        .build();
    /// ```
    pub fn set_logger<F>(mut self, logger: F) -> Self
    where
        F: Fn(ULLogLevel, &str) + 'static,
    {
        logger::set_logger(logger);
        self
    }

    /// Enables the platform file system.
    ///
    /// This will use the default file system handler. This MUST be called before creating any other Ultralight objects. This will panic if called more than once.
    pub fn enable_platform_file_system(mut self) -> Self {
        if self.is_file_system_set {
            panic!("File system handler already set!");
        }
        unsafe { ultralight_sys::ulEnablePlatformFileSystem(ptr::null_mut()) };
        self.is_file_system_set = true;
        self
    }

    /// Enables the platform file system with a custom path prefix.
    ///
    /// This will use the default file system handler. This MUST be called before creating any other Ultralight objects. This will panic if called more than once.
    pub fn enable_platform_file_system_with_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        if self.is_file_system_set {
            panic!("File system handler already set!");
        }
        let path = crate::string::ULString::new(path.as_ref().to_str().unwrap());
        unsafe { ultralight_sys::ulEnablePlatformFileSystem(*path) };
        self.is_file_system_set = true;
        self
    }

    /// Enables the platform font loader.
    ///
    /// This will use the default font loader. This MUST be called before creating any other Ultralight objects. This will panic if called more than once.
    pub fn enable_platform_font_loader(mut self) -> Self {
        unsafe { ultralight_sys::ulEnablePlatformFontLoader() };
        self.is_font_loader_set = true;
        self
    }

    /// Enables a custom GPU driver.
    ///
    /// This will use the default GPU driver. It is currently unimplemented.
    pub fn enable_gpu_driver(mut self) -> Self {
        todo!("GPU driver not implemented yet");
    }

    /// Builds the platform.
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
