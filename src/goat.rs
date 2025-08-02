use std::path::PathBuf;
use std::collections::HashMap;
use std::fs::{self, DirEntry};
use anyhow::{anyhow};
use crate::cache::Cache;
use crate::config::Config;
use crate::package_manager::PackageManager;

/// The Goat struct represents the system in whole.
pub struct Goat {
    /// Important directories in a map for lookup & more.
    directories: HashMap<String, PathBuf>,
    
    /// Important files in a map for lookup & more.
    /// 
    /// The idea is to have the directory section joined with 
    /// a file from the files hashmap. This allows for custom
    /// directory setups later.
    files: HashMap<String, PathBuf>,
    
    cache: Cache,
    
    package_manager: PackageManager,
    
    config: Config
}

/// Generate a config.lua file
pub fn generate_system_config(package_manager: &PackageManager, path: &PathBuf) -> anyhow::Result<()> {
    let explicit_packages: String = package_manager
        .explicit_packages()?
        .iter()
        .map(|package| format!("    \"{}\"", package))
        .collect::<Vec<String>>()
        .join(",\n");
    
    fs::write(path, format!("packages = {{\n{}\n}}", explicit_packages))?;
    
    Ok(())
}

impl Goat {
    pub fn get_directories() -> HashMap<String, PathBuf> {
        if cfg!(debug_assertions) {
            // DEBUG MODE DIRECTORIES
            HashMap::from([
                // Location for user configuration.
                (String::from("configuration_directory"), PathBuf::from("example")),
                // Location for cache files and such.
                (String::from("cache_directory"), PathBuf::from("test_cache")),
                // Location of package manager configuration files.
                (String::from("package_manager_configuration_directory"), PathBuf::from("package_managers")),
            ])
        } else {
            HashMap::from([
                // Location for user configuration.
                (String::from("configuration_directory"), PathBuf::from("/etc/goat")),
                // Location for cache files and such.
                (String::from("cache_directory"), PathBuf::from("/var/goat/cache")),
                // Location of package manager configuration files.
                (String::from("package_manager_configuration_directory"), PathBuf::from("/var/goat/package_managers")),
            ])
        }
    }
    
    pub fn get_files() -> HashMap<String, PathBuf> {
        HashMap::from([
            (String::from("config_file"), PathBuf::from("config.lua")),
            (String::from("cache_file"), PathBuf::from("cache.json"))
        ])
    }

    pub fn check_directories(directories: &HashMap<String, PathBuf>) -> anyhow::Result<()> {
        // Confirm all directories exist
        // and are actually directories.
        for directory in directories.values() {
            if !directory.exists() || !directory.is_dir() {
                log::warn!("Directory \"{}\" doesn't exist! Fixing...", directory.display());
                fs::create_dir_all(directory)?;
            }
        }

        Ok(())
    }

    /// Initialize the goat struct and confirm system vitals.
    /// 
    /// Running with the recache parameter set to true
    /// will reset the cache files.
    pub fn load(recache: bool) -> anyhow::Result<Self> {
        // Start: System health check
        let directories = Self::get_directories();
        let files = Self::get_files();
        
        Self::check_directories(&directories)?;
        // log::info!("System health check complete!");
        
        // End: System health check

        // Check for cached package manager value to skip
        // reading all configurations
        
        let cache_file = directories["cache_directory"].join(&files["cache_file"]);
        if !cache_file.exists() || recache {
            log::warn!("Recaching \"{}\"...", cache_file.display());
            fs::write(&cache_file, "{}\n")?;
        }

        let mut cache = Cache::load_cache(&cache_file)?;
        let mut package_manager: Option<PackageManager> = None;

        match cache.package_manager_configuration_file {
            Some(ref file_name) => {
                // If we have a cached config name we will just automatically use that.
                package_manager = Some(PackageManager::from_file(&directories["package_manager_configuration_directory"].join(file_name))?);
            },
            None => {
                log::warn!("Package manager config name not cached, detecting package manager and caching.");

                // Get a list of every config file in the package manager configuration directory.
                let package_manager_configuration_paths: Vec<DirEntry>
                    = directories["package_manager_configuration_directory"]
                        .read_dir()?
                        .collect::<Result<_, _>>()?;

                for package_manager_configuration_path in package_manager_configuration_paths.iter() {
                    let package_manager_test = PackageManager::from_file(&package_manager_configuration_path.path())?;
                    if which::which(&package_manager_test.binary_name).is_ok() {
                        cache.package_manager_configuration_file = Some(package_manager_configuration_path
                                                                        .file_name()
                                                                        .into_string()
                                                                        .map_err(|_| anyhow!("Failed to convert OsString to String. UTF-8?"))?);
                        package_manager = Some(package_manager_test);
                    }
                }
            }
        };

        let package_manager = package_manager.ok_or_else(|| anyhow!("Failed to locate applicable package manager configuration file"))?;
        
        // Dump cache back into cache file.
        cache.save_cache(&cache_file)?;
        
        let config_file = directories["configuration_directory"].join(&files["config_file"]);
        if !config_file.exists() {
            log::warn!("Generating configuration file \"{}\"...", cache_file.display());
            generate_system_config(&package_manager, &config_file)?;
        }
        
        let config = Config::from_file(&config_file)?;
        
        Ok(Goat {
            directories,
            files,
            cache,
            package_manager,
            config
        })
    }
    
}
