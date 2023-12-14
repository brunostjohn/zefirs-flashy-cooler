use super::{
    internal_string::JSCString,
    types::{IsJSValue, JSObject, JSOpaque},
};
use crate::{context::JSContext, value::JSValue};
use libffi::high::Closure6;
use std::{mem, ptr};

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

    pub fn make_function<'b: 'a, S, F, V, E>(
        &mut self,
        name: S,
        callback: F,
    ) -> JSValue<JSObject<'a>>
    where
        S: AsRef<str>,
        F: (Fn(
                JSValue<JSObject<'_>>,
                JSValue<JSObject<'_>>,
                &[JSValue<JSOpaque<'_>>],
            ) -> Result<JSValue<V>, JSValue<E>>)
            + 'b,
        V: IsJSValue,
        E: IsJSValue,
    {
        let name = JSCString::new(name, self.internal.guard);
        let wrapped_function = move |ctx: ultralight_sys::JSContextRef,
                                     function: ultralight_sys::JSObjectRef,
                                     this: ultralight_sys::JSObjectRef,
                                     argument_count: usize,
                                     arguments: *const ultralight_sys::JSValueRef,
                                     exception: *mut ultralight_sys::JSValueRef|
              -> ultralight_sys::JSValueRef {
            let evil_ctx =
                JSContext::new(ctx, unsafe { mem::transmute(ptr::null::<u8>()) }, unsafe {
                    mem::transmute(ptr::null::<u8>())
                });
            let arguments = unsafe { std::slice::from_raw_parts(arguments, argument_count) };
            let arguments = arguments
                .iter()
                .map(|value| {
                    JSValue::new(JSOpaque {
                        internal: *value,
                        guard: &evil_ctx,
                    })
                })
                .collect::<Vec<_>>();
            let this = JSValue::new(JSObject {
                internal: this,
                guard: &evil_ctx,
            });
            let function = JSValue::new(JSObject {
                internal: function,
                guard: &evil_ctx,
            });
            let result = callback(this, function, arguments.as_slice());
            match result {
                Ok(value) => value.internal.get_value(),
                Err(value) => {
                    unsafe {
                        *exception = value.internal.get_value();
                    }
                    ptr::null_mut()
                }
            }
        };
        let closure = Closure6::new(&wrapped_function);
        let code_ptr: &'b unsafe extern "C" fn(
            ultralight_sys::JSContextRef,
            ultralight_sys::JSObjectRef,
            ultralight_sys::JSObjectRef,
            usize,
            *const ultralight_sys::JSValueRef,
            *mut ultralight_sys::JSValueRef,
        ) -> ultralight_sys::JSValueRef = unsafe { mem::transmute(closure.code_ptr()) };
        let function = unsafe {
            ultralight_sys::JSObjectMakeFunctionWithCallback(
                **self.internal.guard,
                *name,
                Some(*code_ptr),
            )
        };

        JSValue::new(JSObject {
            internal: function,
            guard: self.internal.guard,
        })
    }
}
