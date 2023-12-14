use std::ops::Deref;

use crate::context::JSContext;

pub(crate) struct JSCString<'a> {
    pub(crate) internal: ultralight_sys::JSStringRef,
    #[allow(unused)]
    pub(crate) guard: &'a JSContext<'a>,
}

impl Deref for JSCString<'_> {
    type Target = ultralight_sys::JSStringRef;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl From<JSCString<'_>> for String {
    fn from(string: JSCString<'_>) -> Self {
        let internal = string.internal;

        let length = unsafe { ultralight_sys::JSStringGetMaximumUTF8CStringSize(internal) };
        let mut buffer = vec![0; length];

        unsafe {
            ultralight_sys::JSStringGetUTF8CString(internal, buffer.as_mut_ptr() as *mut i8, length)
        };

        String::from_utf8(buffer).unwrap()
    }
}

impl<'a> JSCString<'a> {
    pub(crate) fn from_raw(
        internal: ultralight_sys::JSStringRef,
        guard: &'a JSContext<'a>,
    ) -> Self {
        Self { internal, guard }
    }

    pub(crate) fn new<S: AsRef<str>>(string: S, guard: &'a JSContext<'a>) -> Self {
        let internal = unsafe {
            ultralight_sys::JSStringCreateWithUTF8CString(string.as_ref().as_ptr() as *const i8)
        };

        Self { internal, guard }
    }
}

impl<'a> Drop for JSCString<'a> {
    fn drop(&mut self) {
        unsafe { ultralight_sys::JSStringRelease(**self) }
    }
}
