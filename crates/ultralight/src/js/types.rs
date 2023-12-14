use crate::{context::JSContext, ULError};

pub trait IsJSValue {
    fn get_value(&self) -> ultralight_sys::JSValueRef;
}

pub struct JSOpaque<'a> {
    pub(crate) internal: ultralight_sys::JSValueRef,
    #[allow(unused)]
    pub(crate) guard: &'a JSContext<'a>,
}

impl IsJSValue for JSOpaque<'_> {
    fn get_value(&self) -> ultralight_sys::JSValueRef {
        self.internal
    }
}

pub struct JSFunction<'a> {
    pub(crate) internal: ultralight_sys::JSValueRef,
    #[allow(unused)]
    pub(crate) guard: &'a JSContext<'a>,
}

impl IsJSValue for JSFunction<'_> {
    fn get_value(&self) -> ultralight_sys::JSValueRef {
        self.internal
    }
}

pub struct JSString<'a> {
    pub(crate) internal: ultralight_sys::JSValueRef,
    #[allow(unused)]
    pub(crate) guard: &'a JSContext<'a>,
}

impl IsJSValue for JSString<'_> {
    fn get_value(&self) -> ultralight_sys::JSValueRef {
        self.internal
    }
}

pub struct JSUndefined<'a> {
    pub(crate) internal: ultralight_sys::JSValueRef,
    #[allow(unused)]
    pub(crate) guard: &'a JSContext<'a>,
}

impl IsJSValue for JSUndefined<'_> {
    fn get_value(&self) -> ultralight_sys::JSValueRef {
        self.internal
    }
}

pub struct JSNull<'a> {
    pub(crate) internal: ultralight_sys::JSValueRef,
    #[allow(unused)]
    pub(crate) guard: &'a JSContext<'a>,
}

impl IsJSValue for JSNull<'_> {
    fn get_value(&self) -> ultralight_sys::JSValueRef {
        self.internal
    }
}

pub struct JSBoolean<'a> {
    pub(crate) internal: ultralight_sys::JSValueRef,
    #[allow(unused)]
    pub(crate) guard: &'a JSContext<'a>,
}

impl IsJSValue for JSBoolean<'_> {
    fn get_value(&self) -> ultralight_sys::JSValueRef {
        self.internal
    }
}

pub struct JSNumber<'a> {
    pub(crate) internal: ultralight_sys::JSValueRef,
    #[allow(unused)]
    pub(crate) guard: &'a JSContext<'a>,
}

impl IsJSValue for JSNumber<'_> {
    fn get_value(&self) -> ultralight_sys::JSValueRef {
        self.internal
    }
}

pub struct JSObject<'a> {
    pub(crate) internal: ultralight_sys::JSValueRef,
    #[allow(unused)]
    pub(crate) guard: &'a JSContext<'a>,
}

impl IsJSValue for JSObject<'_> {
    fn get_value(&self) -> ultralight_sys::JSValueRef {
        self.internal
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JSType {
    Undefined,
    Null,
    Boolean,
    Number,
    String,
    Object,
    Opaque,
}

impl TryFrom<ultralight_sys::JSType> for JSType {
    type Error = ULError;
    fn try_from(value: ultralight_sys::JSType) -> Result<Self, Self::Error> {
        match value {
            ultralight_sys::JSType_kJSTypeUndefined => Ok(Self::Undefined),
            ultralight_sys::JSType_kJSTypeNull => Ok(Self::Null),
            ultralight_sys::JSType_kJSTypeBoolean => Ok(Self::Boolean),
            ultralight_sys::JSType_kJSTypeNumber => Ok(Self::Number),
            ultralight_sys::JSType_kJSTypeString => Ok(Self::String),
            ultralight_sys::JSType_kJSTypeObject => Ok(Self::Object),
            _ => Err(ULError::JSValueTypeNotKnown),
        }
    }
}

impl From<JSType> for ultralight_sys::JSType {
    fn from(value: JSType) -> Self {
        match value {
            JSType::Undefined => ultralight_sys::JSType_kJSTypeUndefined,
            JSType::Null => ultralight_sys::JSType_kJSTypeNull,
            JSType::Boolean => ultralight_sys::JSType_kJSTypeBoolean,
            JSType::Number => ultralight_sys::JSType_kJSTypeNumber,
            JSType::String => ultralight_sys::JSType_kJSTypeString,
            JSType::Object => ultralight_sys::JSType_kJSTypeObject,
            JSType::Opaque => panic!("Cannot convert opaque type to JSType"),
        }
    }
}
