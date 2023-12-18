use ultralight_sys::JSGarbageCollect;

use crate::{context::JSContext, ULRendererGuard, ULResult, ULSurface, ULViewBuilder};

use super::{
    guard::{ULViewConfigGuard, ULViewGuard},
    load_future::{LoadFuture, LoadFutureContainer},
};
use std::ptr;

pub struct ULView<'a> {
    pub(crate) internal: ULViewGuard,
    renderer: &'a ULRendererGuard,
    config: ULViewConfigGuard,
}

unsafe impl Send for ULView<'_> {}
unsafe impl Sync for ULView<'_> {}

impl<'a> ULView<'a> {
    pub(crate) unsafe fn from_raw(
        view: ultralight_sys::ULView,
        renderer: &'a ULRendererGuard,
        config: ULViewConfigGuard,
    ) -> Self {
        Self {
            internal: ULViewGuard::new(view),
            renderer,
            config,
        }
    }

    pub fn into_builder(self) -> ULViewBuilder<'a> {
        ULViewBuilder::from_raw_built(*self.internal, self.renderer, Some(self.config))
    }

    pub fn get_url(&self) -> String {
        unsafe {
            let url = ultralight_sys::ulViewGetURL(*self.internal);
            let url = std::ffi::CStr::from_ptr(url as *const _);
            url.to_string_lossy().into_owned()
        }
    }

    pub fn get_title(&self) -> String {
        unsafe {
            let title = ultralight_sys::ulViewGetTitle(*self.internal);
            let title = std::ffi::CStr::from_ptr(title as *const _);
            title.to_string_lossy().into_owned()
        }
    }

    pub fn get_width(&self) -> u32 {
        unsafe { ultralight_sys::ulViewGetWidth(*self.internal) }
    }

    pub fn get_height(&self) -> u32 {
        unsafe { ultralight_sys::ulViewGetHeight(*self.internal) }
    }

    pub fn get_device_scale(&self) -> f64 {
        unsafe { ultralight_sys::ulViewGetDeviceScale(*self.internal) }
    }

    pub fn set_device_scale(&mut self, device_scale: f64) {
        unsafe { ultralight_sys::ulViewSetDeviceScale(*self.internal, device_scale) }
    }

    pub fn is_accelerated(&self) -> bool {
        unsafe { ultralight_sys::ulViewIsAccelerated(*self.internal) }
    }

    pub fn is_transparent(&self) -> bool {
        unsafe { ultralight_sys::ulViewIsTransparent(*self.internal) }
    }

    pub fn is_loading(&self) -> bool {
        unsafe { ultralight_sys::ulViewIsLoading(*self.internal) }
    }

    pub(crate) fn load_url_start<S: AsRef<str>>(&mut self, url: S) {
        let url = crate::string::ULString::new(url.as_ref());
        unsafe { ultralight_sys::ulViewLoadURL(*self.internal, *url) }
    }

    pub async fn load_url<S: AsRef<str>>(&mut self, url: S) -> ULResult<()> {
        self.load_url_start(url);

        LoadFuture {
            renderer: &LoadFutureContainer(**self.renderer),
        }
        .await
    }

    pub(crate) fn load_html_start<S: AsRef<str>>(&mut self, html: S) {
        let html = crate::string::ULString::new(html.as_ref());
        unsafe { ultralight_sys::ulViewLoadHTML(*self.internal, *html) }
    }

    pub async fn load_html<S: AsRef<str>>(&mut self, html: S) -> ULResult<()> {
        self.load_html_start(html);

        LoadFuture {
            renderer: &LoadFutureContainer(**self.renderer),
        }
        .await
    }

    pub(crate) fn lock_js_context(&self) -> ultralight_sys::JSContextRef {
        unsafe { ultralight_sys::ulViewLockJSContext(*self.internal) }
    }

    pub(crate) fn unlock_js_context(&self) {
        unsafe { ultralight_sys::ulViewUnlockJSContext(*self.internal) }
    }

    pub(crate) fn get_render_target_raw(&self) -> ultralight_sys::ULRenderTarget {
        unsafe { ultralight_sys::ulViewGetRenderTarget(*self.internal) }
    }

    pub fn evaluate_script<S: AsRef<str>>(&self, script: S) -> String {
        let script = crate::string::ULString::new(script.as_ref());
        unsafe {
            let result =
                ultralight_sys::ulViewEvaluateScript(*self.internal, *script, ptr::null_mut());
            let result = std::ffi::CStr::from_ptr(result as *const _);
            result.to_string_lossy().into_owned()
        }
    }

    pub fn get_can_go_back(&self) -> bool {
        unsafe { ultralight_sys::ulViewCanGoBack(*self.internal) }
    }

    pub fn get_can_go_forward(&self) -> bool {
        unsafe { ultralight_sys::ulViewCanGoForward(*self.internal) }
    }

    pub fn go_back(&mut self) {
        unsafe { ultralight_sys::ulViewGoBack(*self.internal) }
    }

    pub fn go_forward(&mut self) {
        unsafe { ultralight_sys::ulViewGoForward(*self.internal) }
    }

    pub fn go_to_history_offset(&mut self, offset: i32) {
        unsafe { ultralight_sys::ulViewGoToHistoryOffset(*self.internal, offset) }
    }

    pub fn reload(&mut self) {
        unsafe { ultralight_sys::ulViewReload(*self.internal) }
    }

    pub fn stop(&mut self) {
        unsafe { ultralight_sys::ulViewStop(*self.internal) }
    }

    pub fn focus(&mut self) {
        unsafe { ultralight_sys::ulViewFocus(*self.internal) }
    }

    pub fn unfocus(&mut self) {
        unsafe { ultralight_sys::ulViewUnfocus(*self.internal) }
    }

    pub fn get_has_focus(&self) -> bool {
        unsafe { ultralight_sys::ulViewHasFocus(*self.internal) }
    }

    pub fn create_local_inspector_view(&mut self) {
        unsafe { ultralight_sys::ulViewCreateLocalInspectorView(*self.internal) }
    }

    pub(crate) fn get_render_surface(&mut self) -> ultralight_sys::ULSurface {
        unsafe { ultralight_sys::ulViewGetSurface(*self.internal) }
    }

    pub fn get_surface(&mut self) -> ULSurface {
        unsafe { ULSurface::from_raw(self.get_render_surface()) }
    }

    pub fn get_js_context(&'a mut self) -> JSContext<'a> {
        let ctx = self.lock_js_context();
        JSContext::new(ctx, self.renderer, self)
    }

    pub fn render_png(&mut self) {}

    #[inline]
    pub fn garbage_collect(&mut self) {
        unsafe {
            let ctx = self.lock_js_context();
            JSGarbageCollect(ctx);
            self.unlock_js_context();
        }
    }
}
