use std::ops::{Deref, DerefMut};

pub(crate) struct ULViewGuard(pub(crate) ultralight_sys::ULView);

unsafe impl Sync for ULViewGuard {}
unsafe impl Send for ULViewGuard {}

impl ULViewGuard {
    pub fn new(view: ultralight_sys::ULView) -> Self {
        Self(view)
    }
}

impl Drop for ULViewGuard {
    fn drop(&mut self) {
        unsafe { ultralight_sys::ulDestroyView(self.0) };
    }
}

impl Deref for ULViewGuard {
    type Target = ultralight_sys::ULView;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ULViewGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub struct ULViewConfigGuard(ultralight_sys::ULViewConfig);

impl Default for ULViewConfigGuard {
    fn default() -> Self {
        Self::new()
    }
}

impl ULViewConfigGuard {
    pub fn new() -> Self {
        let config = unsafe { ultralight_sys::ulCreateViewConfig() };
        assert!(!config.is_null());
        Self(config)
    }
}

impl Drop for ULViewConfigGuard {
    fn drop(&mut self) {
        unsafe { ultralight_sys::ulDestroyViewConfig(self.0) };
    }
}

impl Deref for ULViewConfigGuard {
    type Target = ultralight_sys::ULViewConfig;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ULViewConfigGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
