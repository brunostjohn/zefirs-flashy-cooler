use crate::CONFIG;
use crate::SENSORS;
// use windows_taskscheduler::{RunLevel, Task, TaskAction, TaskLogonTrigger};

#[tauri::command]
pub fn get_start_minimised() -> bool {
    let config = CONFIG.lock().unwrap();
    return config.start_minimised;
}

#[tauri::command]
pub fn set_start_minimised(setting: bool) {
    let mut config = CONFIG.lock().unwrap();
    config.start_minimised = setting;
    config.write_to_drive()
}

#[tauri::command]
pub fn get_start_login() -> bool {
    let config = CONFIG.lock().unwrap();
    return config.start_at_login;
}

#[tauri::command]
pub fn set_start_login(setting: bool) {
    // if setting {
    //     let trigger = TaskLogonTrigger::new("zefirs-flashy-cooler-autostart");
    // }
    let mut config = CONFIG.lock().unwrap();
    config.start_minimised = setting;
    config.write_to_drive();
}

#[tauri::command]
pub fn get_poll_rate() -> u64 {
    let config = CONFIG.lock().unwrap();
    config.poll_rate
}

#[tauri::command]
pub fn set_poll_rate(poll_rate: u64) {
    let sensors = SENSORS.lock().unwrap();
    sensors.change_poll_rate(poll_rate);
    let mut config = CONFIG.lock().unwrap();
    config.poll_rate = poll_rate;
    config.write_to_drive();
}
