// goat_lua is meant to hold functions and macros for interactions between rust and lua.
// Things like global functions (that load into the lua runtime) should be placed here.

/// Quickly extract a lua variable and dump 
/// it into the config if it exists.
#[macro_export]
macro_rules! lua_extract_string_variable {
    ($name:ident, $globals:expr, $config:expr) => {
        if let Ok(x) = $globals.get(stringify!($name)) {
            $config.$name = x;
        }
    };
}

/// Check if a program is in PATH
///
/// This function is exported to the lua runtime
pub fn program_exists(program: &str) -> bool{
    match which::which(program) {
        Ok(_) => true,
        Err(_) => false
    }
}

#[macro_export]
macro_rules! include_custom_runtime {
    ($lua:ident, $globals:ident) => {
        {
            let goat_table = $lua.create_table().map_err(|e| anyhow!("{}", e.to_string()))?;
            goat_table.set("program_exists", $lua.create_function(|_, program: String| {
                Ok(goat_lua::program_exists(&program))
            }).unwrap()).map_err(|e| anyhow!("{}", e.to_string()))?;
            $globals.set("goat", goat_table).map_err(|e| anyhow!("{}", e.to_string()))?;
        }
    };
}