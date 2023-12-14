use crate::context::JSContext;

use super::types::{
    IsJSValue, JSBoolean, JSNull, JSNumber, JSObject, JSOpaque, JSString, JSType, JSUndefined,
};

pub struct JSValue<T: IsJSValue> {
    pub(crate) internal: T,
    #[allow(unused)]
    pub(crate) value: ultralight_sys::JSValueRef,
}

impl<T: IsJSValue> JSValue<T> {
    pub(crate) fn new(internal: T) -> Self {
        let value = internal.get_value();
        Self { internal, value }
    }
}

impl<'a> JSValue<JSOpaque<'a>> {
    pub(crate) fn new_opaque(
        internal: ultralight_sys::JSValueRef,
        guard: &'a JSContext<'_>,
    ) -> Self {
        Self {
            internal: JSOpaque { internal, guard },
            value: internal,
        }
    }

    pub fn get_type(&self) -> JSType {
        let guard = self.internal.guard;
        let internal = self.internal.internal;
        let value_type: JSType = unsafe { ultralight_sys::JSValueGetType(**guard, internal) }
            .try_into()
            .unwrap();

        value_type
    }

    pub unsafe fn cast_undefined(self) -> JSValue<JSUndefined<'a>> {
        JSValue::new(JSUndefined {
            internal: self.internal.internal,
            guard: self.internal.guard,
        })
    }

    pub unsafe fn cast_null(self) -> JSValue<JSNull<'a>> {
        JSValue::new(JSNull {
            internal: self.internal.internal,
            guard: self.internal.guard,
        })
    }

    pub unsafe fn cast_boolean(self) -> JSValue<JSBoolean<'a>> {
        JSValue::new(JSBoolean {
            internal: self.internal.internal,
            guard: self.internal.guard,
        })
    }

    pub unsafe fn cast_number(self) -> JSValue<JSNumber<'a>> {
        JSValue::new(JSNumber {
            internal: self.internal.internal,
            guard: self.internal.guard,
        })
    }

    pub unsafe fn cast_string(self) -> JSValue<JSString<'a>> {
        JSValue::new(JSString {
            internal: self.internal.internal,
            guard: self.internal.guard,
        })
    }

    pub unsafe fn cast_object(self) -> JSValue<JSObject<'a>> {
        JSValue::new(JSObject {
            internal: self.internal.internal,
            guard: self.internal.guard,
        })
    }
}
