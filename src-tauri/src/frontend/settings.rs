use std::process::Command;

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
pub fn set_start_login(setting: bool) -> Result<(), &'static str> {
    config.start_at_login = setting;
    config.write_to_drive(app_folder.0.to_owned());

    if setting {
        let argv = [
            "/C",
            &("schtasks /create /RU ".to_string()
                + &whoami::username()
                + " /tn ZefirsFlashyCooler-Autostart /tr "
                + app_folder.0.to_str().unwrap()
                + "\\zefirs-flashy-cooler.exe"
                + " /sc onlogon /IT /RL highest /f"),
        ];

        Command::new("cmd")
            .args(argv)
            .spawn()
            .or(Err("Failed to run command!"))?;
    } else {
        let argv = [
            "/C",
            "schtasks /query | findstr ZefirsFlashyCooler-Autostart",
        ];

        let exit_code = Command::new("cmd")
            .args(argv)
            .output()
            .or(Err("Failed to run command!"))?
            .status;

        if exit_code.success() {
            let argv = ["/C", "schtasks /delete /tn ZefirsFlashyCooler-Autostart /f"];

            Command::new("cmd")
                .args(argv)
                .spawn()
                .or(Err("Failed to run command!"))?;
        }
    }

    Ok(())
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
