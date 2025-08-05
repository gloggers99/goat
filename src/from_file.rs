use std::path::PathBuf;

/// This is to unify types such as PackageManager and ServiceManager behind a reusable interface.
pub trait FromFile {
    fn from_file(path: &PathBuf) -> anyhow::Result<Self> 
        where Self: Sized;
    
    /// Anything that implements FromFile will need to have a `binary_name` field. Or some kind of
    /// string returned from this method.
    fn get_binary_name(&self) -> &str;
}