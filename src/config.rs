use std::path::Path;
use starlark::environment::{GlobalsBuilder, Module};
use starlark::eval::Evaluator;
use starlark::syntax::{AstModule, Dialect};

use crate::{get_str_var, starlark_goat};

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
    packages: Option<Vec<String>>
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
        // This code is identical to PackageManager::from_file with different
        // fields.

        let ast: AstModule = AstModule::parse_file(path, &Dialect::Standard)
            .map_err(|e| format!("Failed to load configuration file \"{}\", because: {}", path.display(), e))?;
        let globals = GlobalsBuilder::new().with(starlark_goat::starlark_pkgs).build();
        let module: Module = Module::new();
        let mut eval: Evaluator = Evaluator::new(&module);
        
        // We use .map_err(...) on result types to map whatever the library deems an `Error`
        // to our functions result type which is a String. So if this function fails it will
        // return Err(String).
        eval.eval_module(ast, &globals)
            .map_err(|e| format!("Failed to evaluate configuration file: \"{}\"", e))?;
        
        let mut config = Config::default();
        
        // If get_str_var returns Ok(&str) we will set the hostname to the
        // specified string.
        if let Ok(hostname) = get_str_var!(module, "hostname", path) {
            config.hostname = hostname.to_owned();
        }

        if let Ok(packages_variable) = get_str_var!(module, "packages", path) {
            config.packages = Some(packages_variable.split(',').map(String::from).collect());
        }
        
        Ok(config)
    }
}