mod goat;
mod package_manager;
mod cache;
mod config;
mod starlark_goat;

use std::process::exit;
use goat::Goat;

fn main() {
    // Show all info level and above log messages in stdout.
    env_logger::Builder::from_env(env_logger::Env::default()
        .default_filter_or("info"))
        .format_timestamp(None)
        .init();

    // Prefixed with _ because it is never used.
    let _system = match Goat::load() {
        Ok(system) => system,
        Err(e) => {
            log::error!("{}", e);
            exit(1);
        }
    };
}
