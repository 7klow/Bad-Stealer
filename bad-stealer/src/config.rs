// src/config.rs
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub watch_paths: Vec<PathBuf>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeywordConfig {
    pub keywords: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        let config_dir = get_config_dir();
        let config_path = config_dir.join("bs_path_config.json");

        if !config_dir.exists() {
            fs::create_dir_all(&config_dir).expect("Failed to create config directory");
        }

        if config_path.exists() {
            let content = fs::read_to_string(&config_path).expect("Failed to read config file");
            serde_json::from_str(&content).unwrap_or_else(|_| Self::default())
        } else {
            let default_config = Self::default();
            let json = serde_json::to_string_pretty(&default_config).unwrap();
            let mut file = File::create(&config_path).expect("Failed to create config file");
            file.write_all(json.as_bytes()).expect("Failed to write config file");
            default_config
        }
    }
}

impl KeywordConfig {
    pub fn new() -> Self {
        let config_dir = get_config_dir();
        let keyword_path = config_dir.join("bs_keyword_config.json");

        if keyword_path.exists() {
            let content = fs::read_to_string(&keyword_path).expect("Failed to read keyword config");
            serde_json::from_str(&content).unwrap_or_else(|_| Self::default())
        } else {
            let default_keywords = Self::default();
            let json = serde_json::to_string_pretty(&default_keywords).unwrap();
            let mut file = File::create(&keyword_path).expect("Failed to create keyword config");
            file.write_all(json.as_bytes()).expect("Failed to write keyword config");
            default_keywords
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let mut paths = Vec::new();

        #[cfg(target_os = "windows")]
        {
            let user_profile = std::env::var("USERPROFILE").unwrap_or_default();
            paths.push(PathBuf::from(format!("{}\\Desktop", user_profile)));
            paths.push(PathBuf::from(format!("{}\\Documents", user_profile)));
            paths.push(PathBuf::from(format!("{}\\Downloads", user_profile)));
            paths.push(PathBuf::from(format!("{}\\AppData\\Local\\Temp", user_profile)));
        }

        #[cfg(target_os = "linux")]
        {
            let home = std::env::var("HOME").unwrap_or_default();
            paths.push(PathBuf::from(format!("{}/Desktop", home)));
            paths.push(PathBuf::from(format!("{}/Documents", home)));
            paths.push(PathBuf::from(format!("{}/Downloads", home)));
            paths.push(PathBuf::from("/tmp"));
        }

        Self { watch_paths: paths }
    }
}

impl Default for KeywordConfig {
    fn default() -> Self {
        Self {
            keywords: vec![
                "discord".into(),
                "webhook".into(),
                "telegram".into(),
                "mega".into(),
                "pastebin".into(),
                "transfer".into(),
                "ftp://".into(),
                "http://".into(),
            ],
        }
    }
}

fn get_config_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        let local_app_data = std::env::var("LOCALAPPDATA")
            .unwrap_or_else(|_| "C:\\Users\\Default\\AppData\\Local".to_string());
        Path::new(&local_app_data).join("BadStealer")
    }

    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        Path::new(&home).join(".config").join("BadStealer")
    }
}

pub fn get_config_dir_str() -> String {
    #[cfg(target_os = "windows")]
    {
        let local_app_data = std::env::var("LOCALAPPDATA")
            .unwrap_or_else(|_| "C:\\Users\\Default\\AppData\\Local".to_string());
        return format!("{}\\BadStealer", local_app_data);
    }

    #[cfg(target_os = "linux")]
    {
        let home = std::env::var("HOME")
            .unwrap_or_else(|_| "/tmp".to_string());
        return format!("{}/.config/BadStealer", home);
    }
}
