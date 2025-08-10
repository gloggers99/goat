use goat_lua::GoatLua;
use anyhow::anyhow;
use goat_lua_macro::FromLuaFile;

// Time to unify systemd and openrc...

#[derive(FromLuaFile)]
pub struct ServiceManager {
    /// This is the name of the application that the service manager relies on. This will not be
    /// used for commands but to confirm the existence of this specific service manager.
    pub binary_name: String,
    /// The command to run to reload the hostname.
    pub hostname_reload_command: String
}
