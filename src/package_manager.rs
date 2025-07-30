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

        let binary_name = module
            .get("binary_name")
            .ok_or_else(|| format!("No 'binary_name' section found in \"{}\"", path.display()))?
            .unpack_str()
            .ok_or_else(|| format!("Failed to parse 'binary_name' value in \"{}\"", path.display()))?;

        let install_command = module
            .get("install_command")
            .ok_or_else(|| format!("No 'install_command' section found in \"{}\"", path.display()))?
            .unpack_str()
            .ok_or_else(|| format!("Failed to parse 'install_command' value in \"{}\"", path.display()))?;

        let full_system_update_command = module
            .get("full_system_update_command")
            .ok_or_else(|| format!("No 'full_system_update_command' section found in \"{}\"", path.display()))?
            .unpack_str()
            .ok_or_else(|| format!("Failed to parse 'full_system_update_command' value in \"{}\"", path.display()))?;

        Ok(PackageManager {
            binary_name: binary_name.to_owned(),
            install_command: install_command.to_owned(),
            full_system_update_command: full_system_update_command.to_owned()
        })
    }
}
