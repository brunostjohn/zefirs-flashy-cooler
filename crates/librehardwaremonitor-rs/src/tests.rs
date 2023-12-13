use super::*;

#[test]
fn sets_params() {
    let mut computer = Computer::new();
    let old_params = computer.get_params();
    let new_params = ComputerParams {
        is_psu_enabled: true,
        is_gpu_enabled: true,
        is_cpu_enabled: true,
        is_motherboard_enabled: true,
        is_memory_enabled: true,
        is_storage_enabled: true,
        is_network_enabled: true,
        is_controller_enabled: true,
        is_battery_enabled: true,
    };
    computer.set_params(new_params);
    let params = computer.get_params();

    drop(computer);

    assert!(
        params.is_psu_enabled
            && params.is_gpu_enabled
            && params.is_cpu_enabled
            && params.is_motherboard_enabled
            && params.is_memory_enabled
            && params.is_storage_enabled
            && params.is_network_enabled
            && params.is_controller_enabled
            && params.is_battery_enabled
            && !old_params.is_psu_enabled
            && !old_params.is_gpu_enabled
            && !old_params.is_cpu_enabled
            && !old_params.is_motherboard_enabled
            && !old_params.is_memory_enabled
            && !old_params.is_storage_enabled
            && !old_params.is_network_enabled
            && !old_params.is_controller_enabled
            && !old_params.is_battery_enabled
    );
}

#[test]
fn gets_hardware() {
    let mut computer = Computer::new_with_params(ComputerParams {
        is_psu_enabled: true,
        is_gpu_enabled: true,
        is_cpu_enabled: true,
        is_motherboard_enabled: true,
        is_memory_enabled: true,
        is_storage_enabled: true,
        is_network_enabled: true,
        is_controller_enabled: true,
        is_battery_enabled: true,
    });

    for mut hardware in computer.iter() {
        let hardware_name = hardware.get_name().unwrap_or("".into());
        println!("Hardware: {hardware_name}\nSensors:");

        for mut sensor in hardware.sensor_iter() {
            let sensor_name = sensor.get_name().unwrap_or("".into());
            let sensor_value = sensor.get_value().unwrap_or(0f32);
            let sensor_type = sensor.get_type();
            println!("\t{sensor_name}\n\t\tvalue: {sensor_value}\n\t\ttype: {:#?}", sensor_type);
        }
    }
}
