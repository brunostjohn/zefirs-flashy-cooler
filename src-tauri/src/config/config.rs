use std::fs;

use serde::{Deserialize, Serialize};

use crate::APP_FOLDER;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub fps: u64,
    pub start_at_login: bool,
    pub start_minimised: bool,
    pub theme_path: Option<String>,
}

impl Config {
    pub fn default() -> Self {
        Config {
            fps: 25,
            start_at_login: false,
            start_minimised: false,
            theme_path: None,
        }
    }

    pub fn load_from_drive() -> Self {
        let mut load_path = APP_FOLDER.clone();
        load_path.push("config.json");

        let loaded_config = {
            let config_string = match fs::read_to_string(load_path) {
                Ok(result) => result,
                Err(_) => {
                    println!("Failed to load config!");
                    "".to_string()
                }
            };

            match serde_json::from_str::<Config>(&config_string) {
                Ok(config) => config,
                Err(_) => {
                    println!("Failed to parse config!");
                    Config::default()
                }
            }
        };

        loaded_config
    }

    pub fn write_to_drive(&self) {
        let mut write_path = APP_FOLDER.clone();
        write_path.push("config.json");

        let serialised = match serde_json::to_string_pretty(self) {
            Ok(serialised) => serialised,
            Err(_) => {
                println!("Failed to serialise config.");
                "{}".to_string()
            }
        };

        match fs::write(write_path, serialised) {
            Ok(_) => {}
            Err(_) => println!("Failed to write config."),
        }
    }
}

// impl Drop for Config {
// }
