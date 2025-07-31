use std::path::Path;
use starlark::environment::{Globals, Module};
use starlark::eval::Evaluator;
use starlark::syntax::{AstModule, Dialect};

use crate::get_str_var;

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
    list_explicit_packages: String
}

impl PackageManager {
    pub fn from_file(path: &Path) -> Result<Self, String> {
        // This is all according to https://docs.rs/starlark/latest/starlark/
        let ast: AstModule = AstModule::parse_file(path, &Dialect::Standard)
            .map_err(|e| format!("Failed to load package manager configuration file \"{}\", because: {}", path.display(), e))?;
        let globals: Globals = Globals::standard();
        let module: Module = Module::new();
        let mut eval: Evaluator = Evaluator::new(&module);

        eval.eval_module(ast, &globals)
            .map_err(|e| format!("Failed to evaluate package manager configuration file: \"{}\"", e))?;

        let binary_name = get_str_var!(module, "binary_name", path)?;
        let install_command = get_str_var!(module, "install_command", path)?;
        let full_system_update_command = get_str_var!(module, "full_system_update_command", path)?;
        let list_explicit_packages = get_str_var!(module, "list_explicit_packages", path)?;

        Ok(PackageManager {
            binary_name: binary_name.to_owned(),
            install_command: install_command.to_owned(),
            full_system_update_command: full_system_update_command.to_owned(),
            list_explicit_packages: list_explicit_packages.to_owned()
        })
    }
}
