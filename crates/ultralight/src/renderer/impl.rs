use ultralight_sys::{ULGamepadAxisEvent, ULGamepadButtonEvent, ULGamepadEvent};

use crate::{
    string::ULString,
    types::{error::ULError, ULResult},
};
use std::{
    ops::{Deref, DerefMut},
    sync::OnceLock,
};

use super::builder::ULConfigGuard;

pub enum ULRendererGuard {
    Owned(ultralight_sys::ULRenderer),
    Borrowed(ultralight_sys::ULRenderer),
}

unsafe impl Send for ULRendererGuard {}
unsafe impl Sync for ULRendererGuard {}

impl Drop for ULRendererGuard {
    fn drop(&mut self) {
        match self {
            Self::Owned(renderer) => unsafe { ultralight_sys::ulDestroyRenderer(*renderer) },
            Self::Borrowed(_) => {}
        }
    }
}

impl Deref for ULRendererGuard {
    type Target = ultralight_sys::ULRenderer;

    fn deref(&self) -> &Self::Target {
        match self {
            Self::Owned(renderer) => renderer,
            Self::Borrowed(renderer) => renderer,
        }
    }
}

impl DerefMut for ULRendererGuard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            Self::Owned(renderer) => renderer,
            Self::Borrowed(renderer) => renderer,
        }
    }
}

pub struct ULRenderer {
    internal: ULRendererGuard,
    config: ULConfigGuard,
}

#[derive(Debug)]
pub(crate) struct RenderContainer {
    renderer: ultralight_sys::ULRenderer,
    config: ultralight_sys::ULConfig,
}

unsafe impl Send for RenderContainer {}
unsafe impl Sync for RenderContainer {}

static RENDERER_EXISTS: OnceLock<RenderContainer> = OnceLock::new();

impl ULRenderer {
    pub(super) fn new(config: ULConfigGuard) -> Self {
        assert!(
            RENDERER_EXISTS.get().is_none(),
            "Only one renderer can exist per process!"
        );
        let renderer = unsafe { ultralight_sys::ulCreateRenderer(*config) };
        assert!(!renderer.is_null(), "Failed to create renderer!");
        RENDERER_EXISTS
            .set(RenderContainer {
                renderer,
                config: *config,
            })
            .expect("Failed to set renderer!");
        Self {
            internal: ULRendererGuard::Owned(renderer),
            config,
        }
    }

    pub(crate) fn new_borrowed(config: ULConfigGuard) -> Self {
        if RENDERER_EXISTS.get().is_none() {
            let renderer = unsafe { ultralight_sys::ulCreateRenderer(*config) };
            assert!(!renderer.is_null(), "Failed to create renderer!");
            RENDERER_EXISTS
                .set(RenderContainer {
                    renderer,
                    config: *config,
                })
                .expect("Failed to set renderer!");

            Self {
                internal: ULRendererGuard::Borrowed(renderer),
                config,
            }
        } else {
            Self::get_existing()
        }
    }

    pub(crate) fn get_existing() -> Self {
        let render_ptr = RENDERER_EXISTS.get().expect("Renderer does not exist!");

        Self {
            internal: ULRendererGuard::Borrowed(render_ptr.renderer),
            config: ULConfigGuard::Borrowed(render_ptr.config),
        }
    }

    pub(crate) fn internal(&self) -> &ULRendererGuard {
        &self.internal
    }

    #[inline(always)]
    pub fn update(&self) {
        unsafe { ultralight_sys::ulUpdate(*self.internal) }
    }

    #[inline(always)]
    pub fn render(&self) {
        unsafe { ultralight_sys::ulRender(*self.internal) }
    }

    pub fn purge_memory(&self) {
        unsafe { ultralight_sys::ulPurgeMemory(*self.internal) }
    }

    pub fn log_memory_usage(&self) {
        unsafe { ultralight_sys::ulLogMemoryUsage(*self.internal) }
    }

    pub fn start_remote_inspector_server<S: AsRef<str>>(
        &self,
        address: S,
        port: u16,
    ) -> ULResult<()> {
        let address = address.as_ref();
        let res = unsafe {
            ultralight_sys::ulStartRemoteInspectorServer(
                *self.internal,
                address.as_ptr() as *const _,
                port,
            )
        };

        if res {
            Ok(())
        } else {
            Err(ULError::FailedToStartInspectorServer)
        }
    }

    pub fn set_gamepad_details<S: AsRef<str>>(
        &self,
        index: u32,
        id: S,
        axis_count: u32,
        button_count: u32,
    ) {
        let id = ULString::new(id);
        unsafe {
            ultralight_sys::ulSetGamepadDetails(
                *self.internal,
                index,
                *id,
                axis_count,
                button_count,
            )
        }
    }

    pub unsafe fn fire_raw_gamepad_event(&self, event: ULGamepadEvent) {
        unsafe { ultralight_sys::ulFireGamepadEvent(*self.internal, event) }
    }

    pub unsafe fn fire_raw_gamepad_axis_event(&self, event: ULGamepadAxisEvent) {
        unsafe { ultralight_sys::ulFireGamepadAxisEvent(*self.internal, event) }
    }

    pub unsafe fn fire_raw_gamepad_button_event(&self, event: ULGamepadButtonEvent) {
        unsafe { ultralight_sys::ulFireGamepadButtonEvent(*self.internal, event) }
    }
}
