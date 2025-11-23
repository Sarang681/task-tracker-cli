use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    storage: StorageConfig,
}

#[derive(Deserialize, Debug)]
struct StorageConfig {
    directory: String,
}

pub fn fetch_json_file_path_from_config() -> Result<String, Box<dyn std::error::Error>> {
    let config_str = fs::read_to_string("Config.toml")?;
    let config: Config = toml::from_str(&config_str)?;

    let mut storage_directory = config.storage.directory;
    if storage_directory.is_empty() {
        storage_directory = String::from("./tasks.json");
    } else {
        storage_directory.push_str("/tasks.json");
    }
    Ok(storage_directory)
}
