//! The main way to interact with Ultralight. This module contains the main types and functions for interacting with the Ultralight renderer.
//!
//! # Example
//! ```rust
//! use ultralight::{platform::ULPlatformBuilder, renderer::ULRendererBuilder};
//!
//! let platform = ULPlatformBuilder::new()
//!   .enable_platform_file_system()
//!   .enable_platform_font_loader()
//!   .build();
//!
//! let renderer = ULRendererBuilder::new()
//!   .build();
//! ```

mod builder;
mod r#impl;

pub use builder::*;
pub use r#impl::*;

#[cfg(test)]
mod tests;
