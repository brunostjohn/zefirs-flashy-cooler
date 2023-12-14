use std::{ops::Deref, ptr};

use super::types::{JSBoolean, JSNull, JSNumber, JSObject, JSString, JSUndefined};
use crate::{value::JSValue, ULRendererGuard, ULView};

pub struct JSContext<'a> {
    pub(crate) internal: ultralight_sys::JSContextRef,
    #[allow(unused)]
    pub(crate) renderer: &'a ULRendererGuard,
    pub(crate) view: &'a ULView<'a>,
}

impl<'a> Deref for JSContext<'a> {
    type Target = ultralight_sys::JSContextRef;

    fn deref(&self) -> &Self::Target {
        &self.internal
    }
}

impl<'a> Drop for JSContext<'a> {
    fn drop(&mut self) {
        unsafe {
            ultralight_sys::ulViewUnlockJSContext(*self.view.internal);
        }
    }
}

impl<'a> JSContext<'a> {
    pub(crate) fn new(
        internal: ultralight_sys::JSContextRef,
        renderer: &'a ULRendererGuard,
        view: &'a ULView<'a>,
    ) -> Self {
        Self {
            internal,
            renderer,
            view,
        }
    }

    pub fn get_global_object(&'a self) -> JSValue<JSObject<'a>> {
        let value_ptr = unsafe { ultralight_sys::JSContextGetGlobalObject(**self) };

        unsafe { JSValue::new_opaque(value_ptr, self).cast_object() }
    }

    pub fn make_null(&'a self) -> JSValue<JSNull<'a>> {
        let value_ptr = unsafe { ultralight_sys::JSValueMakeNull(**self) };

        unsafe { JSValue::new_opaque(value_ptr, self).cast_null() }
    }

    pub fn make_undefined(&'a self) -> JSValue<JSUndefined<'a>> {
        let value_ptr = unsafe { ultralight_sys::JSValueMakeUndefined(**self) };

        unsafe { JSValue::new_opaque(value_ptr, self).cast_undefined() }
    }

    pub fn make_boolean(&'a self, boolean: bool) -> JSValue<JSBoolean<'a>> {
        let value_ptr = unsafe { ultralight_sys::JSValueMakeBoolean(**self, boolean) };

        unsafe { JSValue::new_opaque(value_ptr, self).cast_boolean() }
    }

    pub fn make_number(&'a self, number: f64) -> JSValue<JSNumber<'a>> {
        let value_ptr = unsafe { ultralight_sys::JSValueMakeNumber(**self, number) };

        unsafe { JSValue::new_opaque(value_ptr, self).cast_number() }
    }

    pub fn make_string<S: AsRef<str>>(&'a self, string: S) -> JSValue<JSString<'a>> {
        let string = std::ffi::CString::new(string.as_ref()).unwrap();
        let js_string = unsafe { ultralight_sys::JSStringCreateWithUTF8CString(string.as_ptr()) };
        let value_ptr = unsafe { ultralight_sys::JSValueMakeString(**self, js_string as _) };
        unsafe { ultralight_sys::JSStringRelease(js_string) };

        unsafe { JSValue::new_opaque(value_ptr, self).cast_string() }
    }

    pub fn make_object(&'a self) -> JSValue<JSObject<'a>> {
        let value_ptr =
            unsafe { ultralight_sys::JSObjectMake(**self, ptr::null_mut(), ptr::null_mut()) };

        unsafe { JSValue::new_opaque(value_ptr, self).cast_object() }
    }
}
