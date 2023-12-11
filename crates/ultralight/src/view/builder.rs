#![allow(unused_mut)]

use std::ptr;
use ultralight_sys::{ulViewSetFailLoadingCallback, ulViewSetFinishLoadingCallback};

use super::guard::ULViewConfigGuard;
use crate::{
    view::load_future::{done_loading, failed_loading},
    ULRenderer, ULRendererGuard, ULView,
};

pub struct ULViewBuilder<'a> {
    internal: ULViewConfigGuard,
    renderer: &'a ULRendererGuard,
    width: u32,
    height: u32,
    built: Option<ultralight_sys::ULView>,
}

impl<'a> ULViewBuilder<'a> {
    pub fn new(renderer: &'a ULRenderer) -> Self {
        Self {
            internal: ULViewConfigGuard::new(),
            renderer: renderer.internal(),
            width: 0,
            height: 0,
            built: None,
        }
    }

    pub fn set_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn set_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }

    pub fn set_is_accelerated(mut self, is_accelerated: bool) -> Self {
        unsafe {
            ultralight_sys::ulViewConfigSetIsAccelerated(*self.internal, is_accelerated);
        }

        self
    }

    pub fn set_is_transparent(mut self, is_transparent: bool) -> Self {
        unsafe {
            ultralight_sys::ulViewConfigSetIsTransparent(*self.internal, is_transparent);
        }

        self
    }

    pub fn set_initial_device_scale(mut self, initial_device_scale: f64) -> Self {
        unsafe {
            ultralight_sys::ulViewConfigSetInitialDeviceScale(*self.internal, initial_device_scale);
        }

        self
    }

    pub fn set_initial_focus(mut self, is_focused: bool) -> Self {
        unsafe {
            ultralight_sys::ulViewConfigSetInitialFocus(*self.internal, is_focused);
        }

        self
    }

    pub fn set_enable_images(mut self, enable_images: bool) -> Self {
        unsafe {
            ultralight_sys::ulViewConfigSetEnableImages(*self.internal, enable_images);
        }

        self
    }

    pub fn set_enable_javascript(mut self, enable_javascript: bool) -> Self {
        unsafe {
            ultralight_sys::ulViewConfigSetEnableJavaScript(*self.internal, enable_javascript);
        }

        self
    }

    pub fn set_font_family_standard<S: AsRef<str>>(mut self, font_family: S) -> Self {
        let font_family = crate::string::ULString::new(font_family.as_ref());
        unsafe {
            ultralight_sys::ulViewConfigSetFontFamilyStandard(*self.internal, *font_family);
        }

        self
    }

    pub fn set_font_family_fixed<S: AsRef<str>>(mut self, font_family: S) -> Self {
        let font_family = crate::string::ULString::new(font_family.as_ref());
        unsafe {
            ultralight_sys::ulViewConfigSetFontFamilyFixed(*self.internal, *font_family);
        }

        self
    }

    pub fn set_font_family_serif<S: AsRef<str>>(mut self, font_family: S) -> Self {
        let font_family = crate::string::ULString::new(font_family.as_ref());
        unsafe {
            ultralight_sys::ulViewConfigSetFontFamilySerif(*self.internal, *font_family);
        }

        self
    }

    pub fn set_font_family_sans_serif<S: AsRef<str>>(mut self, font_family: S) -> Self {
        let font_family = crate::string::ULString::new(font_family.as_ref());
        unsafe {
            ultralight_sys::ulViewConfigSetFontFamilySansSerif(*self.internal, *font_family);
        }

        self
    }

    pub fn set_user_agent<S: AsRef<str>>(mut self, user_agent: S) -> Self {
        let user_agent = crate::string::ULString::new(user_agent.as_ref());
        unsafe {
            ultralight_sys::ulViewConfigSetUserAgent(*self.internal, *user_agent);
        }

        self
    }

    pub fn from_raw_built(
        view: ultralight_sys::ULView,
        renderer: &'a ULRendererGuard,
        config: Option<ULViewConfigGuard>,
    ) -> Self {
        Self {
            internal: config.unwrap_or_default(),
            renderer,
            width: 0,
            height: 0,
            built: Some(view),
        }
    }

    pub fn build(mut self) -> ULView<'a> {
        unsafe {
            if self.built.is_none() {
                self.built = Some(ultralight_sys::ulCreateView(
                    **self.renderer,
                    self.width,
                    self.height,
                    *self.internal,
                    ptr::null_mut(),
                ));
            }
            if let Some(view) = self.built {
                assert!(!view.is_null());
                ulViewSetFinishLoadingCallback(view, Some(done_loading), ptr::null_mut());
                ulViewSetFailLoadingCallback(view, Some(failed_loading), ptr::null_mut());
                ULView::from_raw(view, self.renderer, self.internal)
            } else {
                panic!("View already built")
            }
        }
    }
}
