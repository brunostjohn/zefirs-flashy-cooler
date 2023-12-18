//! Utilities for working with bitmaps.
//!
//! # Example
//! ```rust
//! use ultralight::{Bitmap, BitmapFormat};
//!
//! let bitmap = Bitmap::create_borrowed(100, 100, BitmapFormat::Bgra8UnormSrgb).unwrap();
//! assert_eq!(bitmap.width(), 100);
//! ```

mod format;
mod r#impl;
mod pixels;

pub use format::*;
pub use pixels::*;
pub use r#impl::*;
