#[repr(C)]
pub enum HardwareType {
    Motherboard,
    SuperIO,
    Cpu,
    Memory,
    GpuNvidia,
    GpuAmd,
    GpuIntel,
    Storage,
    Network,
    Cooler,
    EmbeddedController,
    Psu,
    Battery,
}

#[repr(C)]
pub enum SensorType {
    Voltage,
    Current,
    Power,
    Clock,
    Temperature,
    Load,
    Frequency,
    Fan,
    Flow,
    Control,
    Level,
    Factor,
    Data,
    SmallData,
    Throughput,
    TimeSpan,
    Energy,
    Noise,
}

#[link(name = "LibreHardwareMonitorNative", kind = "static")]
extern "C" {
    pub fn create_computer_object() -> i32;
    pub fn set_computer_is_psu_enabled(computer_id: i32, enabled: bool);
    pub fn get_computer_is_psu_enabled(computer_id: i32) -> bool;
    pub fn set_computer_is_gpu_enabled(computer_id: i32, enabled: bool);
    pub fn get_computer_is_gpu_enabled(computer_id: i32) -> bool;
    pub fn set_computer_is_cpu_enabled(computer_id: i32, enabled: bool);
    pub fn get_computer_is_cpu_enabled(computer_id: i32) -> bool;
    pub fn set_computer_is_motherboard_enabled(computer_id: i32, enabled: bool);
    pub fn get_computer_is_motherboard_enabled(computer_id: i32) -> bool;
    pub fn set_computer_is_memory_enabled(computer_id: i32, enabled: bool);
    pub fn get_computer_is_memory_enabled(computer_id: i32) -> bool;
    pub fn set_computer_is_storage_enabled(computer_id: i32, enabled: bool);
    pub fn get_computer_is_storage_enabled(computer_id: i32) -> bool;
    pub fn set_computer_is_network_enabled(computer_id: i32, enabled: bool);
    pub fn get_computer_is_network_enabled(computer_id: i32) -> bool;
    pub fn set_computer_is_controller_enabled(computer_id: i32, enabled: bool);
    pub fn get_computer_is_controller_enabled(computer_id: i32) -> bool;
    pub fn set_computer_is_battery_enabled(computer_id: i32, enabled: bool);
    pub fn get_computer_is_battery_enabled(computer_id: i32) -> bool;
    pub fn update_computer_object(computer_id: i32);
    pub fn reset_computer_object(computer_id: i32);
    pub fn destroy_computer_object(computer_id: i32);
    pub fn free_dotnet_string(string: *mut u16);
    pub fn get_computer_report(computer_id: i32) -> *mut u16;
    pub fn get_computer_hardware_len(computer_id: i32) -> i32;
    pub fn get_hardware_type(
        computer_id: i32,
        indices_arr_ptr: *mut i32,
        indices_arr_len: i32,
    ) -> HardwareType;
    pub fn get_hardware_name(
        computer_id: i32,
        indices_arr_ptr: *mut i32,
        indices_arr_len: i32,
    ) -> *mut u16;
    pub fn set_hardware_name(
        computer_id: i32,
        indices_arr_ptr: *mut i32,
        indices_arr_len: i32,
        name: *mut u16,
    ) -> i32;
    pub fn get_sensors_len_hardware(
        computer_id: i32,
        indices_arr_ptr: *mut i32,
        indices_arr_len: i32,
    ) -> i32;
    pub fn get_subhardware_len_hardware(
        computer_id: i32,
        indices_arr_ptr: *mut i32,
        indices_arr_len: i32,
    ) -> i32;
    pub fn update_hardware_object(
        computer_id: i32,
        indices_arr_ptr: *mut i32,
        indices_arr_len: i32,
    );
    pub fn get_hardware_report(
        computer_id: i32,
        indices_arr_ptr: *mut i32,
        indices_arr_len: i32,
    ) -> *mut u16;
    pub fn get_sensor_value(
        computer_id: i32,
        indices_arr_ptr: *mut i32,
        indices_arr_len: i32,
        sensor_index: i32,
    ) -> f32;
    pub fn get_sensor_name(
        computer_id: i32,
        indices_arr_ptr: *mut i32,
        indices_arr_len: i32,
        sensor_index: i32,
    ) -> *mut u16;
    pub fn set_sensor_name(
        computer_id: i32,
        indices_arr_ptr: *mut i32,
        indices_arr_len: i32,
        sensor_index: i32,
        name: *mut u16,
    ) -> i32;
    pub fn get_sensor_type(
        computer_id: i32,
        indices_arr_ptr: *mut i32,
        indices_arr_len: i32,
        sensor_index: i32,
    ) -> SensorType;
    pub fn get_min_value_sensor(
        computer_id: i32,
        indices_arr_ptr: *mut i32,
        indices_arr_len: i32,
        sensor_index: i32,
    ) -> f32;
    pub fn get_max_value_sensor(
        computer_id: i32,
        indices_arr_ptr: *mut i32,
        indices_arr_len: i32,
        sensor_index: i32,
    ) -> f32;
    pub fn clear_sensor_values(
        computer_id: i32,
        indices_arr_ptr: *mut i32,
        indices_arr_len: i32,
        sensor_index: i32,
    );
}
