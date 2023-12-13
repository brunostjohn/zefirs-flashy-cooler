use crate::{Computer, HardwareType, LibreError, LibreResult, HardwareIter, SensorIter};

pub struct Hardware<'a> {
    pub(crate) guard: &'a Computer,
    pub(crate) indices: Vec<i32>,
}

impl<'a> Hardware<'a> {
    pub(crate) fn new(guard: &'a Computer, indices: Vec<i32>) -> Self {
        Self { guard, indices }
    }

    pub fn subhardware_iter(&mut self) -> HardwareIter<'_> {
        HardwareIter {
            inner: self,
            index: 0,
            len: self.get_subhardware_len(),
        }
    }

    pub fn get_name(&mut self) -> Option<String> {
        let name_ptr = unsafe {
            librehardwaremonitor_sys::get_hardware_name(
                self.guard.id,
                self.indices.as_mut_ptr(),
                self.indices.len() as i32,
            )
        };
        if name_ptr.is_null() {
            None
        } else {
            let name_cstr = unsafe { std::ffi::CStr::from_ptr(name_ptr as _) };
            let name = name_cstr.to_str().ok()?.to_owned();

            Some(name)
        }
    }

    pub fn set_name(&mut self, name: &str) -> LibreResult<()> {
        let name = std::ffi::CString::new(name).or(Err(LibreError::InvalidName))?;
        let result = unsafe {
            librehardwaremonitor_sys::set_hardware_name(
                self.guard.id,
                self.indices.as_mut_ptr(),
                self.indices.len() as i32,
                name.as_ptr() as _,
            )
        };

        if result == -1 {
            Err(LibreError::FailedToSetName)
        } else {
            Ok(())
        }
    }

    pub fn get_type(&mut self) -> HardwareType {
        let hardware_type = unsafe {
            librehardwaremonitor_sys::get_hardware_type(
                self.guard.id,
                self.indices.as_mut_ptr(),
                self.indices.len() as i32,
            )
        };

        hardware_type.into()
    }

    pub fn get_sensors_len(&self) -> usize {
        unsafe {
            librehardwaremonitor_sys::get_sensors_len_hardware(
                self.guard.id,
                self.indices.as_ptr() as _,
                self.indices.len() as i32,
            )
        }
        .try_into()
        .unwrap()
    }

    pub fn get_subhardware_len(&self) -> usize {
        unsafe {
            librehardwaremonitor_sys::get_subhardware_len_hardware(
                self.guard.id,
                self.indices.as_ptr() as _,
                self.indices.len() as i32,
            )
        }
        .try_into()
        .unwrap()
    }

    pub fn sensor_iter(&self) -> SensorIter<'_> {
        SensorIter {
            inner: self,
            index: 0,
            len: self.get_sensors_len(),
        }
    }

    pub fn update(&mut self) {
        unsafe {
            librehardwaremonitor_sys::update_hardware_object(
                self.guard.id,
                self.indices.as_ptr() as _,
                self.indices.len() as i32,
            )
        };
    }

    pub fn get_report(&mut self) -> Option<String> {
        let report_ptr = unsafe {
            librehardwaremonitor_sys::get_hardware_report(
                self.guard.id,
                self.indices.as_mut_ptr(),
                self.indices.len() as i32,
            )
        };
        if report_ptr.is_null() {
            None
        } else {
            let report_cstr = unsafe { std::ffi::CStr::from_ptr(report_ptr as _) };
            let report = report_cstr.to_str().ok()?.to_owned();

            Some(report)
        }
    }

    pub fn get_parent(&mut self) -> Option<Hardware<'a>> {
        let mut parent_indices = self.indices.clone();
        let parent_indices_len = parent_indices.len();
        if parent_indices_len < 2 {
            None
        } else {
            parent_indices.truncate(parent_indices_len - 1);

            Some(Hardware::new(self.guard, parent_indices))
        }
    }
}
