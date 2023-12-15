use ultralight_sys::JSValueRef;

use crate::value::JSValue;

use super::types::{IsJSValue, JSFunction, JSOpaque};

impl<'a> JSValue<JSFunction<'a>> {
    pub fn call<T>(
        &mut self,
        arguments: &[JSValue<T>],
    ) -> Result<Option<JSValue<JSOpaque<'_>>>, JSValue<JSOpaque<'_>>>
    where
        T: IsJSValue,
    {
        let arguments: Vec<_> = arguments
            .iter()
            .map(|value| value.internal.get_value())
            .collect();
        let mut exception = 0 as JSValueRef;

        let returned = unsafe {
            ultralight_sys::JSObjectCallAsFunction(
                **self.internal.guard,
                self.internal.get_value() as _,
                self.value as _,
                arguments.len() as _,
                arguments.as_ptr(),
                &mut exception,
            )
        };

        if exception.is_null() {
            if returned.is_null() {
                Ok(None)
            } else {
                Ok(Some(JSValue::new(JSOpaque {
                    internal: returned,
                    guard: self.internal.guard,
                })))
            }
        } else {
            Err(JSValue::new(JSOpaque {
                internal: exception,
                guard: self.internal.guard,
            }))
        }
    }
}
