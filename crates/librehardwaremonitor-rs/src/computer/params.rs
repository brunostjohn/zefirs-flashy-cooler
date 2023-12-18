use super::r#impl::Computer;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ComputerParams {
    pub is_psu_enabled: bool,
    pub is_gpu_enabled: bool,
    pub is_cpu_enabled: bool,
    pub is_motherboard_enabled: bool,
    pub is_memory_enabled: bool,
    pub is_storage_enabled: bool,
    pub is_network_enabled: bool,
    pub is_controller_enabled: bool,
    pub is_battery_enabled: bool,
}

impl Computer {
    pub fn get_params(&mut self) -> ComputerParams {
        ComputerParams {
            is_psu_enabled: unsafe {
                librehardwaremonitor_sys::get_computer_is_psu_enabled(self.id)
            },
            is_gpu_enabled: unsafe {
                librehardwaremonitor_sys::get_computer_is_gpu_enabled(self.id)
            },
            is_cpu_enabled: unsafe {
                librehardwaremonitor_sys::get_computer_is_cpu_enabled(self.id)
            },
            is_motherboard_enabled: unsafe {
                librehardwaremonitor_sys::get_computer_is_motherboard_enabled(self.id)
            },
            is_memory_enabled: unsafe {
                librehardwaremonitor_sys::get_computer_is_memory_enabled(self.id)
            },
            is_storage_enabled: unsafe {
                librehardwaremonitor_sys::get_computer_is_storage_enabled(self.id)
            },
            is_network_enabled: unsafe {
                librehardwaremonitor_sys::get_computer_is_network_enabled(self.id)
            },
            is_controller_enabled: unsafe {
                librehardwaremonitor_sys::get_computer_is_controller_enabled(self.id)
            },
            is_battery_enabled: unsafe {
                librehardwaremonitor_sys::get_computer_is_battery_enabled(self.id)
            },
        }
    }

    pub fn set_params(&mut self, params: ComputerParams) {
        let ComputerParams {
            is_psu_enabled,
            is_gpu_enabled,
            is_cpu_enabled,
            is_motherboard_enabled,
            is_memory_enabled,
            is_storage_enabled,
            is_network_enabled,
            is_controller_enabled,
            is_battery_enabled,
        } = params;

        unsafe {
            librehardwaremonitor_sys::set_computer_is_psu_enabled(self.id, is_psu_enabled);
            librehardwaremonitor_sys::set_computer_is_gpu_enabled(self.id, is_gpu_enabled);
            librehardwaremonitor_sys::set_computer_is_cpu_enabled(self.id, is_cpu_enabled);
            librehardwaremonitor_sys::set_computer_is_motherboard_enabled(
                self.id,
                is_motherboard_enabled,
            );
            librehardwaremonitor_sys::set_computer_is_memory_enabled(self.id, is_memory_enabled);
            librehardwaremonitor_sys::set_computer_is_storage_enabled(self.id, is_storage_enabled);
            librehardwaremonitor_sys::set_computer_is_network_enabled(self.id, is_network_enabled);
            librehardwaremonitor_sys::set_computer_is_controller_enabled(
                self.id,
                is_controller_enabled,
            );
            librehardwaremonitor_sys::set_computer_is_battery_enabled(self.id, is_battery_enabled);
        }
    }
}
