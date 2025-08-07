use std::collections::{HashSet};
use anyhow::anyhow;
use mlua::{Lua, Value};
use std::process::Command;
use from_lua_file_macro::FromLuaFile;
use crate::{goat_lua};

#[derive(FromLuaFile)]
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
    
    /// Uninstall a package.
    remove_command: String,
    
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
    list_explicit_packages_command: String,
    
    /// Command to get a list of ALL packages installed on the system. Including those not
    /// explicitly installed.
    list_all_packages_command: String,
    
    /// A list of packages REQUIRED to be installed by the package manager.
    /// 
    /// ex: pacman has "core" and "linux"/"linux-zen"
    core_packages: Vec<String>
}

impl PackageManager {
/// Get a Vec<String> of explicitly installed packages.
    pub fn explicit_packages(&self) -> anyhow::Result<Vec<String>> {
        let parts = self.list_explicit_packages_command.split_whitespace();
        let args: Vec<&str> = parts.collect();
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(&self.list_explicit_packages_command)
            .output()?;
        
        // Theoretically this should be safe unless the package
        // manager's output is something weird like non UTF-8.
        let stdout = String::from_utf8(output.stdout)?;
        
        Ok(stdout.split_whitespace().map(|x| x.to_owned()).collect())
    }
    
    pub fn all_packages(&self) -> anyhow::Result<Vec<String>> {
        let parts: Vec<&str> = self.list_all_packages_command.split_whitespace().collect();
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(&self.list_all_packages_command)
            .output()?;
        
        let stdout = String::from_utf8(output.stdout)?;
        
        Ok(stdout.split_whitespace().map(|x| x.to_owned()).collect())
    }
    
    /// Install a list of packages using the PackageManager specification
    pub fn install(&self, packages: Vec<&str>) -> anyhow::Result<()> {
        let command_parts = self.full_system_update_command.split_whitespace();
        
        // Filter out already installed packages
        let installed_packages: HashSet<String> = self.all_packages()?.into_iter().collect();
        
        let packages: Vec<&str> = packages
            .into_iter()
            .filter(|package| !installed_packages.contains(*package))
            .collect();
        
        if packages.is_empty() {
            log::info!("No new packages.");
            return Ok(())
        }
        
        log::info!("Installing {} package(s): {}.", packages.len(), packages.join(","));
        
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
            return Err(anyhow!("Package installation failed with output: \n\n{}", String::from_utf8(output.stderr)?))
        }
        
        Ok(())
    }

    pub fn remove_unneeded_packages(&self, explicitly_needed_packages: Vec<&str>) -> anyhow::Result<()> {
        let explicitly_installed_packages: HashSet<String> = self.explicit_packages()?.into_iter().collect();

        let needed: HashSet<&str> = explicitly_needed_packages.iter().copied().collect();
        let unneeded_packages: Vec<&str> = explicitly_installed_packages
            .iter()
            .filter(|pkg| !needed.contains(pkg.as_str()))
            .map(|s| s.as_str())
            .collect();
        
        if unneeded_packages.is_empty() {
            return Ok(())
        }

        log::info!("Removing {} package(s): {}.", unneeded_packages.len(), unneeded_packages.join(","));

        // Join all packages into a single space-separated string
        let packages = unneeded_packages.join(" ");

        // Format the command string with the packages
        let command_str = self.remove_command.replace("{}", &packages);

        let mut parts = command_str.split_whitespace();
        let cmd = parts.next().ok_or_else(|| anyhow!("Removal command is empty"))?;
        let args: Vec<&str> = parts.collect();

        let output = Command::new(cmd)
            .args(&args)
            .output()
            .map_err(|e| anyhow!("Failed to execute removal command: {}", e))?;

        if !output.status.success() {
            return Err(anyhow!("Package removal failed with output: \n\n{}", String::from_utf8(output.stderr)?))
        }

        Ok(())
    }
}
