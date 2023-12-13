use crate::{Computer, Hardware, LibreError, LibreResult, SensorType};

pub struct Sensor<'a> {
    pub(crate) computer_guard: &'a Computer,
    pub(crate) hardware_guard: &'a Hardware<'a>,
    pub(crate) index: i32,
}

impl<'a> Sensor<'a> {
    pub fn get_name(&mut self) -> Option<String> {
        let name_ptr = unsafe {
            librehardwaremonitor_sys::get_sensor_name(
                self.computer_guard.id,
                self.hardware_guard.indices.as_ptr() as _,
                self.hardware_guard.indices.len() as i32,
                self.index,
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
            librehardwaremonitor_sys::set_sensor_name(
                self.computer_guard.id,
                self.hardware_guard.indices.as_ptr() as _,
                self.hardware_guard.indices.len() as i32,
                self.index,
                name.as_ptr() as _,
            )
        };

        if result == -1 {
            Err(LibreError::FailedToSetName)
        } else {
            Ok(())
        }
    }

    pub fn get_value(&mut self) -> LibreResult<f32> {
        let value = unsafe {
            librehardwaremonitor_sys::get_sensor_value(
                self.computer_guard.id,
                self.hardware_guard.indices.as_ptr() as _,
                self.hardware_guard.indices.len() as i32,
                self.index,
            )
        };

        if value == -1f32 {
            Err(LibreError::FailedToGetSensorValue)
        } else {
            Ok(value)
        }
    }

    pub fn get_type(&mut self) -> SensorType {
        let sensor_type = unsafe {
            librehardwaremonitor_sys::get_sensor_type(
                self.computer_guard.id,
                self.hardware_guard.indices.as_ptr() as _,
                self.hardware_guard.indices.len() as i32,
                self.index,
            )
        };

        sensor_type.into()
    }

    pub fn get_min_value(&mut self) -> f32 {
        unsafe {
            librehardwaremonitor_sys::get_min_value_sensor(
                self.computer_guard.id,
                self.hardware_guard.indices.as_ptr() as _,
                self.hardware_guard.indices.len() as i32,
                self.index,
            )
        }
    }

    pub fn get_max_value(&mut self) -> f32 {
        unsafe {
            librehardwaremonitor_sys::get_max_value_sensor(
                self.computer_guard.id,
                self.hardware_guard.indices.as_ptr() as _,
                self.hardware_guard.indices.len() as i32,
                self.index,
            )
        }
    }

    pub fn clear_sensor_values(&mut self) {
        unsafe {
            librehardwaremonitor_sys::clear_sensor_values(
                self.computer_guard.id,
                self.hardware_guard.indices.as_ptr() as _,
                self.hardware_guard.indices.len() as i32,
                self.index,
            )
        };
    }

    pub fn get_parent(&mut self) -> Hardware<'a> {
        Hardware::new(self.computer_guard, self.hardware_guard.indices.clone())
    }
}
