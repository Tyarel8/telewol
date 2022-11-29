use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

fn get_devices_path(file_name: &str) -> PathBuf {
    if cfg!(debug_assertions) {
        PathBuf::from(file_name)
    } else {
        let exe_path = std::env::current_exe().unwrap();
        exe_path.parent().unwrap().join(file_name)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub telegram_token: String,
    pub computers: HashMap<String, String>,
}

impl Config {
    pub fn load() -> Result<Self, &'static str> {
        let config_path = get_devices_path("telewol.toml");
        let Ok(config_file) = std::fs::read_to_string(config_path) else {
         return Err(   "Unable to read config file, should be in the same directory as the executable")
        };

        if let Ok(config) = toml::from_str(&config_file) {
            Ok(config)
        } else {
            Err("Unable to parse config file")
        }
    }

    pub fn save(&self) -> Result<(), &'static str> {
        let config_path = get_devices_path("telewol.toml");
        let Ok(config_file) = toml::to_string(&self) else {
            return Err("Unable to serialize config");
        };

        if std::fs::write(config_path, config_file).is_ok() {
            Ok(())
        } else {
            Err("Unable to write config file")
        }
    }

    pub fn list_computers(&self) -> String {
        if self.computers.is_empty() {
            return "No computers, add them using `/add <computer> <MAC>`".to_string();
        }
        let mut computers = String::new();
        for (name, mac) in &self.computers {
            computers.push_str(&format!("{} => {}\n", name, mac));
        }
        computers
    }
}
