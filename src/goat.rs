use std::string::String;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs::{self, DirEntry};
use anyhow::{anyhow};
use nix::unistd::Uid;
use crate::cache::Cache;
use crate::config::Config;
use crate::from_file::FromFile;
use crate::package_manager::PackageManager;
use crate::service_manager::ServiceManager;

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
    service_manager: ServiceManager,
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
    
    fs::write(path, format!("hostname=\"{}\"\n\npackages = {{\n{}\n}}\n", fs::read_to_string("/etc/hostname")?.trim(), explicit_packages))?;
    
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
                // Location of service manager configuration files.
                (String::from("service_manager_configuration_directory"), PathBuf::from("service_managers"))
            ])
        } else {
            HashMap::from([
                // Location for user configuration.
                (String::from("configuration_directory"), PathBuf::from("/etc/goat")),
                // Location for cache files and such.
                (String::from("cache_directory"), PathBuf::from("/var/goat/cache")),
                // Location of package manager configuration files.
                (String::from("package_manager_configuration_directory"), PathBuf::from("/var/goat/package_managers")),
                // Location of service manager configuration files.
                (String::from("service_manager_configuration_directory"), PathBuf::from("/var/goat/service_managers"))
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
        // Confirm all directories exist and are actually directories.
        for directory in directories.values() {
            if !directory.exists() || !directory.is_dir() {
                log::warn!("Directory \"{}\" doesn't exist! Fixing...", directory.display());
                fs::create_dir_all(directory)?;
            }
        }

        // Bonus check!
        if !Path::new("/etc/hostname").exists() {
            log::warn!("/etc/hostname doesn't exist! Are you on windows?");
        }

        Ok(())
    }

    /// Helper function to load cached file names for things like:
    ///  - Package manager configuration files.
    ///  - Service manager configuration files.
    /// 
    /// If this function succeeds it will return a tuple of type `(T, Option<String>)`, the latter
    /// being the path of the new cached config file if applicable. If a cached file already exists
    /// It will return `(<value of type T>, None)`. Upon failure this will return `anyhow::Err`
    pub fn from_cached_file<T: FromFile>(optional_file_path: &Option<String>, 
                                         configuration_directory: &Path) -> anyhow::Result<(T, Option<String>)> {
        match optional_file_path {
            Some(file_path) => Ok((T::from_file(&configuration_directory.join(file_path))?, None)),
            None => {
                // Get a list of every config file in the package manager configuration directory.
                let package_manager_configuration_paths: Vec<DirEntry>
                    = configuration_directory
                    .read_dir()?
                    .collect::<Result<_, _>>()?;

                for package_manager_configuration_path in package_manager_configuration_paths.iter() {
                    let package_manager_test = T::from_file(&package_manager_configuration_path.path())?;
                    if which::which(&package_manager_test.get_binary_name()).is_ok() {
                        return Ok((
                            package_manager_test, 
                            Some(package_manager_configuration_path.file_name()
                                                                   .into_string()
                                                                   .map_err(|_| anyhow!("Failed to convert OsString to String. UTF-8?"))?)
                        ));
                    }
                }
                
                Err(anyhow!("No package manager configuration found"))
            }
        }
    }
    
    /// Initialize the goat struct and confirm system vitals.
    /// 
    /// Running with the recache parameter set to true
    /// will reset the cache files.
    pub fn load(recache: bool) -> anyhow::Result<Self> {
        if !Uid::effective().is_root() {
            return Err(anyhow!("Goat must be ran with root privileges!"));
        }
        
        let directories = Self::get_directories();
        let files = Self::get_files();
        
        Self::check_directories(&directories)?;

        // Check for cached package manager value to skip reading all configurations
        
        let cache_file = directories["cache_directory"].join(&files["cache_file"]);
        if !cache_file.exists() || recache {
            log::warn!("Recaching \"{}\"...", cache_file.display());
            fs::write(&cache_file, "{}\n")?;
        }

        let mut cache = Cache::load_cache(&cache_file)?;
        
        // TODO: Macroify these 2 let match statements.
        
        let package_manager: PackageManager = match Self::from_cached_file(
            &cache.package_manager_configuration_file,
            &directories["package_manager_configuration_directory"]
        ) {
            Ok((package_manager, None)) => package_manager,
            Ok((package_manager, Some(file_path))) => {
                // Dump cache back into cache file. Originally we did this no matter what before 
                // loading the config file, but now we only write when needed.
                cache.package_manager_configuration_file = Some(file_path);
                cache.save_cache(&cache_file)?;
                
                package_manager
            }
            Err(e) => return Err(anyhow!(e))
        };
        
        let service_manager: ServiceManager = match Self::from_cached_file(
            &cache.service_manager_configuration_file,
            &directories["service_manager_configuration_directory"]
        ) {
            Ok((service_manager, None)) => service_manager,
            Ok((service_manager, Some(file_path))) => {
                cache.service_manager_configuration_file = Some(file_path);
                cache.save_cache(&cache_file)?;
                
                service_manager
            }
            Err(e) => return Err(anyhow!(e))
        };
        
        let config_file = directories["configuration_directory"].join(&files["config_file"]);
        if !config_file.exists() {
            log::warn!("Generating configuration file \"{}\"...", config_file.display());
            generate_system_config(&package_manager, &config_file)?;
        }
        
        let config = Config::from_file(&config_file)?;
        
        Ok(Goat {
            directories,
            files,
            cache,
            package_manager,
            service_manager,
            config
        })
    }
    
    /// This is where 99% of the magic happens.
    pub fn sync(&self) -> anyhow::Result<()> {
        // TODO: We don't want a halfway synced system so in the future we need to containerize our
        //       sync so if an error is thrown we cancel the build and have no side effects.
        
        let current_hostname = fs::read_to_string("/etc/hostname")?.trim().to_owned();
        if current_hostname != self.config.hostname {
            fs::write(Path::new("/etc/hostname"), format!("{}\n", self.config.hostname))?;

            log::warn!("Hostname changed, this will take effect next reboot. See issue #1 on github.")
            // TODO: Do some testing on changing the hostname with systemd as it tends to break 
            //       lots of things like the X server and dbus.
            
            //if !Command::new("sh").arg("-c").arg(&self.service_manager.hostname_reload_command).output()?.status.success() {
            //    return Err(anyhow!("Failed to run \"sh -c {}\"", self.service_manager.hostname_reload_command))
            //}
        }
        
        if let Some(packages) = &self.config.packages {
            let packages: Vec<&str> = packages.iter().map(|package| package.as_str()).collect();
            
            self.package_manager.install(packages.clone())?;
            self.package_manager.remove_unneeded_packages(packages)?;
        }
        
        Ok(())
    }
}
