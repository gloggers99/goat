use anyhow::anyhow;
use mlua::{Lua, Table};
use goat_lua_macro::lua_module;

/// The goat lua runtime module.
#[lua_module]
pub fn goat(lua: &Lua) -> anyhow::Result<Table> {
    pub fn program_exists(program: &str) -> bool {
        match which::which(program) {
            Ok(_) => true,
            Err(_) => false
        }
    }
}

/// The goat_lua runtime
pub struct GoatLua {
    pub lua: Lua
}

impl GoatLua {
    pub fn create() -> anyhow::Result<Self> {
        let lua = Lua::new();

        lua.globals().set("goat", goat(&lua)?).map_err(|e| anyhow!("{}", e))?;

        Ok(Self {
            lua
        })
    }
}
