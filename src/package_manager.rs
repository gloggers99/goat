use std::path::Path;
use anyhow::anyhow;
use log::warn;
use mlua::Lua;

pub struct PackageManager {
    /// The name of any applicable package manager binary.
    ///
    /// This will NOT be used for install/update/etc. commands
    /// as distros like Void Linux have several different package
    /// manager binaries. (xbps-install)
    pub binary_name: String,

    /// The command to install a package, should be this format:
    /// 
    /// `pacman -S {}`
    install_command: String,
    
    /// The command to update & upgrade the whole system
    /// 
    /// ex: `pacman -Syu`
    /// or: `apt update && apt upgrade`
    full_system_update_command: String,
    
    /// Get a list of explicitly installed packages,
    /// AKA packages the user manually typed the installation 
    /// command for.
    /// 
    /// ex: `pacman -Qe | cut -d ' ' -f1`
    list_explicit_packages: String
}

impl PackageManager {
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let lua = Lua::new();
        
        if !path.exists() {
            return Err(anyhow!("Package manager configuration file: \"{}\" does not exist", path.display()));
        }


        let config_script = std::fs::read_to_string(path)?;
        lua.load(&config_script).exec().map_err(|e| anyhow!("Failed to interpret package manager configuration file: {}", e))?;

        let globals = lua.globals();

        let binary_name = globals.get("binary_name").map_err(|e| anyhow!("{}", e))?;
        let install_command = globals.get("install_command").map_err(|e| anyhow!("{}", e))?;
        let full_system_update_command = globals.get("full_system_update_command").map_err(|e| anyhow!("{}", e))?;
        let list_explicit_packages = globals.get("list_explicit_packages").map_err(|e| anyhow!("{}", e))?;
        
        Ok(PackageManager {
            binary_name,
            install_command,
            full_system_update_command,
            list_explicit_packages
        })
    }
}
