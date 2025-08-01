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
    pub fn load_cache(path: &PathBuf) -> anyhow::Result<Self> {
        // I love map_err
        path.try_exists()?;

        let contents
            = fs::read_to_string(path)?;
        let json_contents: Cache
            = serde_json::from_str(&contents)?;

        Ok(json_contents)
    }

    /// Save cache struct into a given file.
    /// 
    /// This function will create the file if it does not exist.
    pub fn save_cache(&self, path: &PathBuf) -> anyhow::Result<()> {
        Ok(fs::write(
            path,
            serde_json::to_string(&self)?
        )?)
    }
}
