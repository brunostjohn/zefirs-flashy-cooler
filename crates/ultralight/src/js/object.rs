use super::types::{IsJSValue, JSObject, JSOpaque};
use crate::value::JSValue;
use std::ptr;

impl<'a> JSValue<JSObject<'a>> {
    pub fn set_property<K: IsJSValue, V: IsJSValue>(&mut self, key: JSValue<K>, value: JSValue<V>) {
        unsafe {
            ultralight_sys::JSObjectSetProperty(
                **self.internal.guard,
                self.internal.get_value() as _,
                key.internal.get_value() as _,
                value.internal.get_value(),
                ultralight_sys::kJSPropertyAttributeNone.try_into().unwrap(),
                ptr::null_mut(),
            );
        }
    }

    pub fn get_property<K: IsJSValue>(&mut self, key: JSValue<K>) -> JSValue<JSOpaque<'a>> {
        let value = unsafe {
            ultralight_sys::JSObjectGetProperty(
                **self.internal.guard,
                self.internal.get_value() as _,
                key.internal.get_value() as _,
                ptr::null_mut(),
            )
        };

        JSValue::new(JSOpaque {
            internal: value,
            guard: self.internal.guard,
        })
    }
}
