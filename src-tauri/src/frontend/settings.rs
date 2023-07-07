// use windows_taskscheduler::{RunLevel, Task, TaskAction, TaskLogonTrigger};

use macros::inject;

#[inject(config)]
#[tauri::command]
pub fn get_start_minimised() -> bool {
    config.start_minimised
}

#[inject(config, app_folder)]
#[tauri::command]
pub fn set_start_minimised(setting: bool) {
    config.start_minimised = setting;
    config.write_to_drive(app_folder.0.to_owned())
}

#[inject(config)]
#[tauri::command]
pub fn get_start_login() -> bool {
    config.start_at_login
}

#[inject(config, app_folder)]
#[tauri::command]
pub fn set_start_login(setting: bool) {
    // if setting {
    //     let trigger = TaskLogonTrigger::new("zefirs-flashy-cooler-autostart");
    // }
    config.start_minimised = setting;
    config.write_to_drive(app_folder.0.to_owned());
}

#[inject(config)]
#[tauri::command]
pub fn get_poll_rate() -> u64 {
    config.poll_rate
}

#[inject(sensors, config, app_folder)]
#[tauri::command]
pub fn set_poll_rate(poll_rate: u64) {
    sensors.change_poll_rate(poll_rate);
    config.poll_rate = poll_rate;
    config.write_to_drive(app_folder.0.to_owned());
}
