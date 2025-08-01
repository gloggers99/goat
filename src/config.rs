use std::path::Path;
use mlua::{Lua, Value};
// Note
// This file has been heavily annotated for William Chastain
// to learn common patterns I use when programming rust.
// 
// Comments with 2 slashes are annotations, comments with 3 are docstrings.
// JetBrains should color code them for you automatically.

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
    
    // These fields are wrapped in an Option monad.
    // This means they can either be equal to 
    // Some(T) or None, with T being the type within the <>'s.
    // 
    // This is because the user might not explicitly 
    // list their packages/hostname/etc. for whatever reason, and we still
    // want the configuration manager to process the config.
    
    /// The list of packages the user explicitly wants installed.
    /// Dependency packages will be pulled in implicitly by their package
    /// manager.
    pub packages: Option<Vec<String>>
}

// Let's implement the `Default` trait for our Config struct.
//
// Traits are like inheritable classes in rust kind of like C++.
//
// This means if we call `Config::default()` it will return
// a `Config` instance with specified default values from here.
impl Default for Config {
    // `Self` refers to the type `Config` where `self` refers to
    // an instance of Config. `&self` is a borrowed instance of
    // self. Don't worry about move semantics as we don't really 
    // get to into that anywhere in goat's code.
    fn default() -> Self {
        Config {
            hostname: String::from("goatOS"),
            packages: None
        }
    }
}

impl Config {
    // Because this function does not have `self` as its first parameter 
    // you can call `Config::from_file(...)` from anywhere. This is basically a `Config`
    // class constructor like in C++ or Python.
    
    /// Create a `Config` instance from a file path.
    pub fn from_file(path: &Path) -> Result<Self, String> {
        let lua = Lua::new();
        
        if !path.exists() {
            return Err(format!("Config file: \"{}\" does not exist", path.display()))
        }
        
        let config_script = std::fs::read_to_string(path).map_err(|err| format!("{}", err.to_string()))?;
        lua.load(&config_script).exec().map_err(|err| format!("Failed to interpret configuration file: {}", err))?;
        
        let globals = lua.globals();
        
        let mut config = Config::default();
        
        if let Ok(hostname) = globals.get("hostname") {
            config.hostname = hostname;
        }

        if let Ok(packages_value) = globals.get::<Value>("packages").map_err(|e| format!("{}", e)) {
            let packages_list = match packages_value.as_table() {
                Some(packages) => {
                    packages.sequence_values::<String>()
                        .collect::<Result<Vec<_>, _>>()
                        .map_err(|err| format!("{}", err))?
                }
                None => return Err("Expected 'packages' in configuration to be a table".into()),
            };

            config.packages = Some(packages_list);
        } 
        
        Ok(config)
    }
}