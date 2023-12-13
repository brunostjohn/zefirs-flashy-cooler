use std::{
    future::Future,
    os::raw::c_void,
    sync::atomic::{AtomicBool, Ordering},
};

use ultralight_sys::{ulRender, ulUpdate};

use crate::{error::ULError, ULResult};

pub(super) static IS_LOADED: AtomicBool = AtomicBool::new(false);
pub(super) static FAILED_LOADING: AtomicBool = AtomicBool::new(false);

#[allow(unused_variables)]
pub(super) unsafe extern "C" fn done_loading(
    user_data: *mut c_void,
    caller: ultralight_sys::ULView,
    frame_id: u64,
    is_main_frame: bool,
    url: ultralight_sys::ULString,
) {
    IS_LOADED.store(true, Ordering::Release);
}

#[allow(unused_variables)]
pub(super) unsafe extern "C" fn failed_loading(
    user_data: *mut c_void,
    caller: ultralight_sys::ULView,
    frame_id: u64,
    is_main_frame: bool,
    url: ultralight_sys::ULString,
    description: ultralight_sys::ULString,
    error_domain: ultralight_sys::ULString,
    error_code: i32,
) {
    FAILED_LOADING.store(true, Ordering::Release);
}

pub(crate) struct LoadFutureContainer(pub(crate) ultralight_sys::ULRenderer);

unsafe impl Send for LoadFutureContainer {}
unsafe impl Sync for LoadFutureContainer {}

pub struct LoadFuture<'a> {
    pub(super) renderer: &'a LoadFutureContainer,
}

impl<'a> Future for LoadFuture<'a> {
    type Output = ULResult<()>;

    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        if IS_LOADED.load(Ordering::Acquire) {
            IS_LOADED.store(false, Ordering::Release);
            std::task::Poll::Ready(Ok(()))
        } else if FAILED_LOADING.load(Ordering::Acquire) {
            FAILED_LOADING.store(false, Ordering::Release);
            std::task::Poll::Ready(Err(ULError::FailedToLoadWebpage))
        } else {
            cx.waker().wake_by_ref();
            unsafe { ulUpdate(self.renderer.0) };
            // unsafe { ulRender(self.renderer.0) };
            std::task::Poll::Pending
        }
    }
}
