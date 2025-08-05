use std::path::PathBuf;
use anyhow::anyhow;
use mlua::Lua;
use crate::from_file::FromFile;
use crate::goat_lua;
use crate::include_custom_runtime;

// Time to unify systemd and openrc...

pub struct ServiceManager {
    /// This is the name of the application that the service manager relies on. This will not be
    /// used for commands but to confirm the existence of this specific service manager.
    pub binary_name: String,
    /// The command to run to reload the hostname.
    hostname_reload_command: String
}

impl FromFile for ServiceManager {
    fn from_file(path: &PathBuf) -> anyhow::Result<Self> {
        let lua = Lua::new();
        
        if !path.exists() {
            return Err(anyhow!("Service manager configuration file: \"{}\" does not exist", path.display()));
        }
        
        let config_script = std::fs::read_to_string(path)?;
        
        let globals = lua.globals();
        include_custom_runtime!(lua, globals);
        
        lua.load(&config_script).exec().map_err(|e| anyhow!("{}", e))?;

        let binary_name = globals.get("binary_name").map_err(|e| anyhow!("{}", e))?;
        let hostname_reload_command = globals.get("hostname_reload_command").map_err(|e| anyhow!("{}", e))?;
        
        Ok(ServiceManager{
            binary_name,
            hostname_reload_command
        })
    }

    fn get_binary_name(&self) -> &str {
        &self.binary_name
    }
}