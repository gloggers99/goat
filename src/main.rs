mod goat;
mod package_manager;
mod cache;
mod config;
mod service_manager;
mod from_file;
mod sync;
mod stage;

use std::process::exit;
use clap::Parser;
use goat::Goat;

#[derive(Parser, Debug)]
#[command(version, 
          about = "System configuration manager",
          long_about = None)]
struct Args {
    /// Sync the system configuration
    #[arg(short, long)]
    sync: bool,
    
    /// Delete all cache files before processing anything else
    #[arg(short='C', long)]
    recache: bool
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    // Show all info level and above log messages in stdout.
    env_logger::Builder::from_env(env_logger::Env::default()
        .default_filter_or("info"))
        .format_timestamp(None)
        .init();

    let system = match Goat::load(args.recache) {
        Ok(system) => system,
        Err(e) => {
            log::error!("{}", e);
            exit(1);
        }
    };
    
    if args.sync {
        log::info!("Syncing system...");
        system.sync()?;
        log::info!("Sync complete.");
    }
    
    Ok(())
}
