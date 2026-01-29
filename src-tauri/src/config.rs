use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub port: u16,
    pub pin: String,
    pub key_bindings: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        let mut key_bindings = HashMap::new();

        // Guitar/Bass frets
        key_bindings.insert("green".to_string(), "a".to_string());
        key_bindings.insert("red".to_string(), "s".to_string());
        key_bindings.insert("yellow".to_string(), "d".to_string());
        key_bindings.insert("blue".to_string(), "f".to_string());
        key_bindings.insert("orange".to_string(), "g".to_string());

        // Strum
        key_bindings.insert("strum_up".to_string(), "Up".to_string());
        key_bindings.insert("strum_down".to_string(), "Down".to_string());

        // Actions
        key_bindings.insert("starpower".to_string(), "l".to_string());
        key_bindings.insert("whammy".to_string(), ";".to_string());
        key_bindings.insert("start".to_string(), "Enter".to_string());
        key_bindings.insert("select".to_string(), "Escape".to_string());

        // Navigation
        key_bindings.insert("left".to_string(), "Left".to_string());
        key_bindings.insert("right".to_string(), "Right".to_string());
        key_bindings.insert("up".to_string(), "Up".to_string());
        key_bindings.insert("down".to_string(), "Down".to_string());

        // Drums
        key_bindings.insert("drum_red".to_string(), "v".to_string());
        key_bindings.insert("drum_yellow".to_string(), "b".to_string());
        key_bindings.insert("drum_blue".to_string(), "n".to_string());
        key_bindings.insert("drum_orange".to_string(), "m".to_string());
        key_bindings.insert("drum_green".to_string(), "c".to_string());
        key_bindings.insert("drum_kick".to_string(), "Space".to_string());

        Self {
            port: 8080,
            pin: "1234".to_string(),
            key_bindings,
        }
    }
}

// ... imports ...
// We will remove config_path() and let the caller provide the path.

pub fn load_config(path: Option<PathBuf>) -> Config {
    // If path is provided, use it. Otherwise try to use a default or just return default config.
    // Actually, for now let's say if path is None we return default, or we can try to resolve it here using 'directories' crate if we add it.
    // But since we want to use Tauri's path resolver, let's assume the caller ALWAYS handles the path resolution.

    if let Some(p) = path {
        if p.exists() {
            match fs::read_to_string(&p) {
                Ok(contents) => match serde_json::from_str(&contents) {
                    Ok(config) => return config,
                    Err(e) => eprintln!("Failed to parse config: {}", e),
                },
                Err(e) => eprintln!("Failed to read config: {}", e),
            }
        }
    }

    Config::default()
}

pub fn save_config(config: &Config, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(config)?;
    fs::write(path, json)?;
    Ok(())
}
