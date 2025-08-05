use std::collections::HashSet;
use anyhow::anyhow;
use mlua::{Lua, Value};
use std::path::PathBuf;
use std::process::Command;
use crate::{goat_lua, include_custom_runtime};
use crate::from_file::FromFile;

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
    list_explicit_packages: String,
    
    /// A list of packages REQUIRED to be installed by the package manager.
    /// 
    /// ex: pacman has "core" and "linux"/"linux-zen"
    core_packages: Vec<String>
}

impl FromFile for PackageManager {
    fn from_file(path: &PathBuf) -> anyhow::Result<Self> {
        let lua = Lua::new();

        if !path.exists() {
            return Err(anyhow!("Package manager configuration file: \"{}\" does not exist", path.display()));
        }

        let config_script = std::fs::read_to_string(path)?;

        let globals = lua.globals();
        include_custom_runtime!(lua, globals);

        lua.load(&config_script).exec().map_err(|e| anyhow!("Failed to interpret package manager configuration file: {}", e))?;


        let binary_name = globals.get("binary_name").map_err(|e| anyhow!("{}", e))?;
        let install_command = globals.get("install_command").map_err(|e| anyhow!("{}", e))?;
        let full_system_update_command = globals.get("full_system_update_command").map_err(|e| anyhow!("{}", e))?;
        let list_explicit_packages = globals.get("list_explicit_packages").map_err(|e| anyhow!("{}", e))?;

        let mut core_packages: Vec<String> = vec![];

        if let Ok(core_packages_value) = globals.get::<Value>("core_packages") {
            if let Some(core_packages_table) = core_packages_value.as_table() {
                core_packages = core_packages_table.sequence_values::<String>()
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| anyhow!("{}", e))?
            }
        }

        Ok(PackageManager {
            binary_name,
            install_command,
            full_system_update_command,
            list_explicit_packages,
            core_packages
        })
    }

    fn get_binary_name(&self) -> &str {
        &self.binary_name
    }
}
impl PackageManager {
/// Get a Vec<String> of explicitly installed packages.
    pub fn explicit_packages(&self) -> anyhow::Result<Vec<String>> {
        let parts = self.list_explicit_packages.split_whitespace();
        let args: Vec<&str> = parts.collect();
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(&self.list_explicit_packages)
            .output()?;
        
        // Theoretically this should be safe unless the package
        // manager's output is something weird like non UTF-8.
        let stdout = String::from_utf8(output.stdout)?;
        
        Ok(stdout.split_whitespace().map(|x| x.to_owned()).collect())
    }
    
    /// Install a list of packages using the PackageManager specification
    pub fn install(&self, packages: Vec<&str>) -> anyhow::Result<()> {
        let command_parts = self.full_system_update_command.split_whitespace();
        
        // Filter out explicitly installed packages
        let installed_packages: HashSet<String> = self.explicit_packages()?.into_iter().collect();
        
        let packages: Vec<&str> = packages
            .into_iter()
            .filter(|package| !installed_packages.contains(*package))
            .collect();
        // Join all packages into a single space-separated string
        let packages = packages.join(" ");

        // Format the command string with the packages
        let command_str = self.install_command.replace("{}", &packages);

        let mut parts = command_str.split_whitespace();
        let cmd = parts.next().ok_or_else(|| anyhow!("Install command is empty"))?;
        let args: Vec<&str> = parts.collect();

        let output = Command::new(cmd)
            .args(&args)
            .output()
            .map_err(|e| anyhow!("Failed to execute install command: {}", e))?;

        if !output.status.success() {
            return Err(anyhow!("Install command failed with status: {}", output.status));
        }
        
        Ok(())
    }
}
