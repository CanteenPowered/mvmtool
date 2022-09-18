use std::fs;
use std::path;

use serde::{Serialize, Deserialize};

use directories::ProjectDirs;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub key: String,
}

impl Config {
    // Get config directory
    fn config_dir() -> Option<path::PathBuf> {
        match ProjectDirs::from("net", "KritzPowered", "mvmtool") {
            Some(dirs)  => Some(path::PathBuf::from(dirs.config_dir())),
            None        => None,
        }
    }

    // Get config file path
    fn config_path() -> Option<path::PathBuf> {
        let mut path = match Self::config_dir() {
            Some(path)  => path,
            None        => { return None; }
        };

        path.push("config.json");

        Some(path)
    }

    // Create a file with a key
    pub fn create(key: String) -> Self {
        let config = Self { key: key, };

        let path = match Self::config_path() {
            Some(path)  => path,
            None        => { return config; }
        };

        let content = match serde_json::to_string(&config) {
            Ok(v)   => v,
            Err(_)  => { return config; }
        };

        // Make sure parent folders exist
        if let Err(e) = fs::create_dir_all(Self::config_dir().unwrap()) {
            eprintln!("Failed to create config directory: {}", e);
        }

        if let Err(e) = fs::write(path, content) {
            eprintln!("Failed to write config file: {}", e);
        }

        config
    }

    // Load a config from disk if it exists
    pub fn load() -> Option<Self> {
        let path = match Self::config_path() {
            Some(path)  => path,
            None        => { return None; }
        };

        let file = match fs::read_to_string(path) {
            Ok(file)    => file,
            Err(_)      => { return None; }
        };

        match serde_json::from_str::<Self>(&file) {
            Ok(v)   => Some(v),
            Err(e)  => { 
                eprintln!("Failed to parse config file: {}", e);
                None
            }
        }
    }
}
