use std::path::Path;
use anyhow::anyhow;
use mlua::{Lua, Value};
use crate::{goat_lua, include_custom_runtime, lua_extract_string_variable};

/// `goat`'s configuration file specification.
/// 
/// Here lies every configuration option
/// for the goat system.
pub struct Config {
    /// The system's hostname. `systemd` systems define this as 
    /// `/etc/hostname` and provides `hostnamectl`. For this
    /// we will stick with `/etc/hostname` for portability.
    /// 
    /// We default this to "goatOS" later.
    hostname: String,

    /// The list of packages the user explicitly wants installed.
    /// Dependency packages will be pulled in implicitly by their package
    /// manager.
    pub packages: Option<Vec<String>>
}

impl Default for Config {
    fn default() -> Self {
        Config {
            hostname: String::from("goatOS"),
            packages: None
        }
    }
}

impl Config {
    /// Create a `Config` instance from a file path.
    pub fn from_file(path: &Path) -> anyhow::Result<Self> {
        let lua = Lua::new();
        
        if !path.exists() {
            return Err(anyhow!("Config file: \"{}\" does not exist", path.display()))
        }
        
        let config_script = std::fs::read_to_string(path)?;

        let globals = lua.globals();
        include_custom_runtime!(lua, globals);
        
        // The mlua library doesn't seem to be friendly with anyhow so we still need
        // to use map_err on each Result returning function from them.
        lua.load(&config_script).exec().map_err(|e| anyhow!("Failed to interpret configuration file: \n{}\n", e))?;
        
        let mut config = Config::default();
        
        lua_extract_string_variable!(hostname, globals, config);

        if let Ok(packages_value) = globals.get::<Value>("packages").map_err(|e| format!("{}", e)) {
            if let Some(packages_list) = packages_value.as_table() {
                config.packages = Some(packages_list.sequence_values::<String>()
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|err| anyhow!("{}", err))?)
            }
        } 
        
        Ok(config)
    }
}