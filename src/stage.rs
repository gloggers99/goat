use std::fs;
use std::path::{Path, PathBuf};
use anyhow::anyhow;
use mlua::ObjectLike;
use goat_lua::GoatLua;
use crate::goat::Goat;

pub enum StageResult {
    Done,
    Skipped
}

/// Seperating each system management layer as a stage allows for easy debugging and modularity.
pub trait Stage {
    /// The stage's name.
    /// 
    /// This is used for debugging and to inform the user about which stage specifically failed,
    /// which stages are being skipped, and more.
    fn name(&self) -> String;
    
    /// Apply the given stage to the system.
    fn apply(&self, goat: &Goat) -> anyhow::Result<StageResult>;
}

/// A custom stage based on a lua file.
pub struct CustomStage {
    pub path: PathBuf
}

impl Stage for CustomStage {
    fn name(&self) -> String { 
        // If this fails I don't know what to tell you.
        self.path.file_name().unwrap().to_string_lossy().to_string()
    }
    
    fn apply(&self, goat: &Goat) -> anyhow::Result<StageResult> {
        let lua = GoatLua::create()?;
        lua.lua.load(&*self.path).exec().map_err(|e| anyhow::anyhow!("{}", e))?;
        let globals = lua.lua.globals();
        let stage = globals.get::<mlua::Table>("stage").map_err(|e| anyhow!("{}", e))?;
        stage.call_function::<()>("apply", ()).map_err(|e| anyhow!("{}", e))?;
        
        Ok(StageResult::Done)
    }
}

/// Hostname stage.
/// 
/// Synchronize hostname to configuration hostname.
pub struct Hostname {} impl Stage for Hostname {
    fn name(&self) -> String { String::from("Hostname") }
    fn apply(&self, goat: &Goat) -> anyhow::Result<StageResult> {
        let current_hostname = fs::read_to_string("/etc/hostname")
            .map_err(|e| anyhow!("{}", e))?
            .trim()
            .to_owned();
        
        if current_hostname != goat.config.hostname {
            fs::write(Path::new("/etc/hostname"), format!("{}\n", goat.config.hostname))?;

            log::warn!("Hostname changed, this will take effect next reboot. See issue #1 on github.");
            // TODO: Do some testing on changing the hostname with systemd as it tends to break 
            //       lots of things like the X server and dbus.

            //if !Command::new("sh").arg("-c").arg(&self.service_manager.hostname_reload_command).output()?.status.success() {
            //    return Err(anyhow!("Failed to run \"sh -c {}\"", self.service_manager.hostname_reload_command))
            //}
            
            Ok(StageResult::Done)
        } else {
            // Current hostname matches configuration hostname
            Ok(StageResult::Skipped)
        }
    }
}

/// Package stage.
/// 
/// Install packages and remove unneeded packages. This stage will only fail if the package manager
/// functions return an error.
pub struct Packages {} impl Stage for Packages {
    fn name(&self) -> String { String::from("Packages") }
    fn apply(&self, goat: &Goat) -> anyhow::Result<StageResult> {
        if let Some(packages) = &goat.config.packages {
            let packages: Vec<&str> = packages.iter().map(|package| package.as_str()).collect();
            
            goat.package_manager.install(packages.clone())?;
            goat.package_manager.remove_unneeded_packages(packages)?;
            
            Ok(StageResult::Done)
        } else {
            Ok(StageResult::Skipped)
        }
    }
}

/// Shortcut for creating an array of stages
/// 
/// Ex:
/// ```
/// let stages = stages![ Hostname, Packages ];
/// ```
#[macro_export]
macro_rules! stages {
    [$($stage:ident),*] => {
        vec![
            $(
                Box::new($stage {}) as Box<dyn Stage>
            ),*
        ]
    };
}
