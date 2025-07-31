use starlark::environment::GlobalsBuilder;
use starlark::starlark_module;
use starlark::values::list::ListRef;

// This module is used for interacting with the starlark
// code and quality of life functionality.

/// Quickly extract a variable from a starlark module.
#[macro_export]
macro_rules! get_str_var {
    ($module:expr, $var:expr, $path:expr) => {
        $module
            .get($var)
            .ok_or_else(|| format!("No '{}' section found in \"{}\"", $var, $path.display()))?
            .unpack_str()
            .ok_or_else(|| format!("Failed to parse '{}' value in \"{}\"", $var, $path.display()))
    };
}

/// This starlark evaluator library does not provide a way to extract
/// a list. This function is added to the starlark evaluator's scope
/// and allows a way to export a list of packages for us to read.
#[starlark_module]
pub fn starlark_pkgs(builder: &mut GlobalsBuilder) {
    fn pkgs(packages: &ListRef<>) -> starlark::Result<String> {
        // As far as unpacking the package list we have to pray
        // the user only puts starlark strings in the array.
        // TODO: Add real error management.
        let package_vec: Vec<&str> = packages
            .iter()
            .map(|package| package.unpack_str().unwrap()).collect();
        
        Ok(package_vec.join(","))
    }
}

