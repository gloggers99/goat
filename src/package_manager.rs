use std::path::Path;

use starlark::environment::{Globals, Module};
use starlark::eval::Evaluator;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::Value;

pub struct PackageManager {
    /// The name of any applicable package manager binary.
    ///
    /// This will NOT be used for install/update/etc. commands
    /// as distros like Void Linux have several different package
    /// manager binaries. (xbps-install)
    pub binary_name: String,

    pub install_command: String,
    pub full_system_update_command: String
}

/// Quickly extract a variable from a starlark module.
macro_rules! get_str_var {
    ($module:expr, $var:expr, $path:expr) => {
        $module
            .get($var)
            .ok_or_else(|| format!("No '{}' section found in \"{}\"", $var, $path.display()))?
            .unpack_str()
            .ok_or_else(|| format!("Failed to parse '{}' value in \"{}\"", $var, $path.display()))
    };
}

impl PackageManager {
    pub fn from_file(path: &Path) -> Result<PackageManager, String> {
        // This is all according to https://docs.rs/starlark/latest/starlark/
        let ast: AstModule = AstModule::parse_file(path, &Dialect::Standard)
            .map_err(|e| format!("Failed to load package manager configuration file \"{}\", because: {}", path.display(), e))?;
        let globals: Globals = Globals::standard();
        let module: Module = Module::new();
        let mut eval: Evaluator = Evaluator::new(&module);

        let res: Value = eval.eval_module(ast, &globals)
            .map_err(|e| format!("Failed to evaluate package manager configuration file: \"{}\"", e))?;

        let binary_name = get_str_var!(module, "binary_name", path)?;
        let install_command = get_str_var!(module, "install_command", path)?;
        let full_system_update_command = get_str_var!(module, "full_system_update_command", path)?;

        Ok(PackageManager {
            binary_name: binary_name.to_owned(),
            install_command: install_command.to_owned(),
            full_system_update_command: full_system_update_command.to_owned()
        })
    }
}
