use std::ffi::CStr;

use crate::{ComputerHardwareIter, ComputerParams};

pub struct Computer {
    pub(crate) id: i32,
}

impl Default for Computer {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Computer {
    fn drop(&mut self) {
        unsafe { librehardwaremonitor_sys::destroy_computer_object(self.id) };
    }
}

impl Computer {
    pub fn new() -> Self {
        let id = unsafe { librehardwaremonitor_sys::create_computer_object() };

        Self { id }
    }

    pub fn new_with_params(params: ComputerParams) -> Self {
        let id = unsafe { librehardwaremonitor_sys::create_computer_object() };
        let mut new_me = Self { id };

        new_me.set_params(params);

        new_me
    }

    pub fn update(&mut self) {
        unsafe { librehardwaremonitor_sys::update_computer_object(self.id) };
    }

    pub fn reset(&mut self) {
        unsafe { librehardwaremonitor_sys::reset_computer_object(self.id) };
    }

    pub fn get_report(&mut self) -> Option<String> {
        let report_ptr = unsafe { librehardwaremonitor_sys::get_computer_report(self.id) };

        if report_ptr.is_null() {
            None
        } else {
            let report = unsafe { CStr::from_ptr(report_ptr as _) }
                .to_string_lossy()
                .into_owned();

            unsafe { librehardwaremonitor_sys::free_dotnet_string(report_ptr) };

            Some(report)
        }
    }

    pub fn get_hardware_len(&mut self) -> usize {
        unsafe { librehardwaremonitor_sys::get_computer_hardware_len(self.id) }
            .try_into()
            .unwrap()
    }

    pub fn iter(&mut self) -> ComputerHardwareIter {
        let len = self.get_hardware_len();

        ComputerHardwareIter {
            inner: self,
            index: 0,
            len,
        }
    }
}
