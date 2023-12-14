#![allow(unused_mut)]

use std::{
    ops::{Deref, DerefMut},
    path::Path,
};

use ultralight_sys::{
    ulConfigSetAnimationTimerDelay, ulConfigSetBitmapAlignment, ulConfigSetCachePath,
    ulConfigSetFaceWinding, ulConfigSetFontGamma, ulConfigSetFontHinting, ulConfigSetForceRepaint,
    ulConfigSetMaxUpdateTime, ulConfigSetMemoryCacheSize, ulConfigSetMinLargeHeapSize,
    ulConfigSetMinSmallHeapSize, ulConfigSetNumRendererThreads, ulConfigSetOverrideRAMSize,
    ulConfigSetPageCacheSize, ulConfigSetRecycleDelay, ulConfigSetResourcePathPrefix,
    ulConfigSetScrollTimerDelay, ulConfigSetUserStylesheet, ulCreateConfig, ulDestroyConfig,
    ULConfig,
};

use crate::{
    string::ULString,
    types::{face_winding::ULFaceWinding, font_hinting::ULFontHinting},
};

use super::r#impl::ULRenderer;

/// A builder for creating a renderer.
///
/// # Example
/// ```rust
/// use ultralight::{platform::ULPlatformBuilder, renderer::ULRendererBuilder};
///
/// let platform = ULPlatformBuilder::new()
///  .enable_platform_file_system()
///  .enable_platform_font_loader()
///  .build();
///
/// let renderer = ULRendererBuilder::new()
///  .set_page_cache_size(128)
///  .build();
pub struct ULRendererBuilder {
    config: ULConfigGuard,
}

impl Default for ULRendererBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ULRendererBuilder {
    pub fn new() -> Self {
        let config = ULConfigGuard::new();
        Self { config }
    }

    pub fn new_borrowed() -> Self {
        let config = unsafe { ulCreateConfig() };
        let config = ULConfigGuard::Borrowed(config);
        Self { config }
    }

    pub fn from_config(config: ULConfigGuard) -> Self {
        Self { config }
    }

    pub fn from_raw(config: ULConfig) -> Self {
        Self {
            config: ULConfigGuard::Owned(config),
        }
    }

    pub fn set_cache_path<P: AsRef<Path>>(mut self, path: P) -> Self {
        let path_string = ULString::new(path.as_ref().to_str().unwrap());

        unsafe { ulConfigSetCachePath(*self.config, *path_string) };
        self
    }

    pub fn set_resource_path_prefix<P: AsRef<Path>>(mut self, path: P) -> Self {
        let path_string = ULString::new(path.as_ref().to_str().unwrap().to_string() + "\\");

        unsafe { ulConfigSetResourcePathPrefix(*self.config, *path_string) };
        self
    }

    pub fn set_face_winding(mut self, face_winding: ULFaceWinding) -> Self {
        unsafe { ulConfigSetFaceWinding(*self.config, face_winding.into()) };
        self
    }

    pub fn set_font_hinting(mut self, hinting: ULFontHinting) -> Self {
        unsafe { ulConfigSetFontHinting(*self.config, hinting.into()) };
        self
    }

    pub fn set_font_gamma(mut self, gamma: f64) -> Self {
        unsafe { ulConfigSetFontGamma(*self.config, gamma) };
        self
    }

    pub fn set_user_stylesheet(mut self, css: &str) -> Self {
        let css_string = ULString::new(css);

        unsafe { ulConfigSetUserStylesheet(*self.config, *css_string) };
        self
    }

    pub fn set_force_repaint(mut self, enabled: bool) -> Self {
        unsafe { ulConfigSetForceRepaint(*self.config, enabled) };
        self
    }

    pub fn set_animation_timer_delay(mut self, delay_ms: f64) -> Self {
        unsafe { ulConfigSetAnimationTimerDelay(*self.config, delay_ms) };
        self
    }

    pub fn set_scroll_timer_delay(mut self, delay_ms: f64) -> Self {
        unsafe { ulConfigSetScrollTimerDelay(*self.config, delay_ms) };
        self
    }

    pub fn set_recycle_delay(mut self, delay_ms: f64) -> Self {
        unsafe { ulConfigSetRecycleDelay(*self.config, delay_ms) };
        self
    }

    pub fn set_memory_cache_size(mut self, size_mb: usize) -> Self {
        unsafe { ulConfigSetMemoryCacheSize(*self.config, size_mb as u32) };
        self
    }

    pub fn set_page_cache_size(mut self, size_mb: usize) -> Self {
        unsafe { ulConfigSetPageCacheSize(*self.config, size_mb as u32) };
        self
    }

    pub fn set_override_ram_size(mut self, size_mb: usize) -> Self {
        unsafe { ulConfigSetOverrideRAMSize(*self.config, size_mb as u32) };
        self
    }

    pub fn set_min_large_heap_size(mut self, size_mb: usize) -> Self {
        unsafe { ulConfigSetMinLargeHeapSize(*self.config, size_mb as u32) };
        self
    }

    pub fn set_min_small_heap_size(mut self, size_mb: usize) -> Self {
        unsafe { ulConfigSetMinSmallHeapSize(*self.config, size_mb as u32) };
        self
    }

    pub fn set_num_renderer_threads(mut self, num_threads: usize) -> Self {
        unsafe { ulConfigSetNumRendererThreads(*self.config, num_threads as u32) };
        self
    }

    pub fn set_max_update_time_secs(mut self, max_time_secs: f64) -> Self {
        unsafe { ulConfigSetMaxUpdateTime(*self.config, max_time_secs) };
        self
    }

    pub fn set_bitmap_alignment(mut self, alignment: u32) -> Self {
        unsafe { ulConfigSetBitmapAlignment(*self.config, alignment) };
        self
    }

    pub fn build(self) -> ULRenderer {
        ULRenderer::new(self.config)
    }
}

pub enum ULConfigGuard {
    Owned(ULConfig),
    Borrowed(ULConfig),
}

unsafe impl Send for ULConfigGuard {}
unsafe impl Sync for ULConfigGuard {}

impl Drop for ULConfigGuard {
    fn drop(&mut self) {
        match self {
            Self::Owned(config) => unsafe { ulDestroyConfig(*config) },
            Self::Borrowed(_) => {}
        }
    }
}

impl Deref for ULConfigGuard {
    type Target = ULConfig;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Owned(config) => config,
            Self::Borrowed(config) => config,
        }
    }
}

impl DerefMut for ULConfigGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Owned(config) => config,
            Self::Borrowed(config) => config,
        }
    }
}

impl Default for ULConfigGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl ULConfigGuard {
    pub fn new() -> Self {
        let config = unsafe { ulCreateConfig() };
        Self::from_raw(config)
    }

    pub fn from_raw(config: ULConfig) -> Self {
        Self::Owned(config)
    }

    pub fn from_borrowed(config: ULConfig) -> Self {
        Self::Borrowed(config)
    }
}
