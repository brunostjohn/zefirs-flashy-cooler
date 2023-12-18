//! # Ultralight
//!
//! `ultralight` is a WIP Rust wrapper for the [Ultralight](https://ultralig.ht/) web rendering engine.
//! Its goal is to provide a safe and idiomatic Rust interface while maintaining the performance of the
//! underlying Ultralight C API. It is currently in a very early stage of development and is not yet
//! ready for production use. It uses a lot of unsafe code and is not yet fully tested. It is also
//! not yet feature complete. If you are interested in contributing, please see the [GitHub repository](https://github.com/brunostjohn/zefirs-flashy-cooler).
//! As it is part of a larger project, it is in the `crates/ultralight` directory. If there is enough interest, it will be moved to its own repository.

pub mod js;
pub mod platform;
pub mod renderer;
mod string;
mod types;
pub mod view;

pub use js::*;
pub use platform::*;
pub use renderer::*;
pub use types::*;
pub use view::*;
