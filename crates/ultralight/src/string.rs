use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
};

pub struct ULString {
    string: ultralight_sys::ULString,
}

impl Deref for ULString {
    type Target = ultralight_sys::ULString;

    fn deref(&self) -> &Self::Target {
        &self.string
    }
}

impl DerefMut for ULString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.string
    }
}

impl Clone for ULString {
    fn clone(&self) -> Self {
        let string = unsafe { ultralight_sys::ulCreateStringFromCopy(self.string) };
        assert!(!string.is_null());
        Self { string }
    }
}

impl Drop for ULString {
    fn drop(&mut self) {
        unsafe { ultralight_sys::ulDestroyString(self.string) };
    }
}

impl Display for ULString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string =
            unsafe { std::ffi::CStr::from_ptr(ultralight_sys::ulStringGetData(self.string)) };
        write!(f, "{}", string.to_str().unwrap())
    }
}

impl Debug for ULString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string =
            unsafe { std::ffi::CStr::from_ptr(ultralight_sys::ulStringGetData(self.string)) };
        write!(f, "{}", string.to_str().unwrap())
    }
}

impl From<&str> for ULString {
    fn from(string: &str) -> Self {
        Self::new(string)
    }
}

impl From<String> for ULString {
    fn from(string: String) -> Self {
        Self::new(string)
    }
}

impl ULString {
    pub fn empty() -> Self {
        let string = unsafe { ultralight_sys::ulCreateString([].as_mut_ptr()) };
        assert!(!string.is_null());
        Self { string }
    }

    pub fn new<S: AsRef<str>>(string: S) -> Self {
        let string = unsafe {
            ultralight_sys::ulCreateStringUTF8(
                string.as_ref().as_ptr() as *const i8,
                string.as_ref().len(),
            )
        };
        assert!(!string.is_null());
        Self { string }
    }

    pub fn as_str(&self) -> &str {
        unsafe {
            std::ffi::CStr::from_ptr(ultralight_sys::ulStringGetData(self.string))
                .to_str()
                .unwrap()
        }
    }

    pub fn as_ptr(&self) -> *const i8 {
        unsafe { ultralight_sys::ulStringGetData(self.string) }
    }

    pub fn len(&self) -> usize {
        unsafe { ultralight_sys::ulStringGetLength(self.string) }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { ultralight_sys::ulStringIsEmpty(self.string) }
    }

    pub fn from_raw(string: ultralight_sys::ULString) -> Self {
        Self { string }
    }
}
