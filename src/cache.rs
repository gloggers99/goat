use std::{fs, path::PathBuf};

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Cache {
    /// The name of the configuration file
    /// used last by `goat` should be just
    /// the last part excluding parent paths.
    ///
    /// Example: "pacman.lua"
    #[serde(default)]
    pub package_manager_configuration_file: Option<String>
}

impl Cache {
    /// Load cache from a given file.
    pub fn load_cache(path: &PathBuf) -> Result<Self, String> {
        // I love map_err
        path.try_exists().map_err(|e| format!("Cache file doesn't exist: {}", e.to_string()))?;

        let contents
            = fs::read_to_string(path).map_err(|e| format!("Failed to read cache file: {}", e.to_string()))?;
        let json_contents: Cache
            = serde_json::from_str(&contents).map_err(|e| format!("Failed to parse json from cache file: {}", e.to_string()))?;

        Ok(json_contents)
    }

    /// Save cache struct into a given file.
    /// 
    /// This function will create the file if it does not exist.
    pub fn save_cache(&self, path: &PathBuf) -> Result<(), String> {
        match path.try_exists() {
            Ok(_) => {},
            Err(_) => {
                std::fs::File::create(&path).map_err(|e| format!("Failed to save cache file: {}", e.to_string()))?;
            }
        };

        std::fs::write(
            path,
            serde_json::to_string(&self).map_err(|e| format!("Failed to save cache file: {}", e.to_string()))?
        ).map_err(|e| format!("Failed to save cache file: {}", e))
    }
}
