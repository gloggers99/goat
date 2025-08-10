use std::fs;
use std::path::Path;
use anyhow::anyhow;
use nix::unistd::Uid;
use crate::goat::Goat;
use crate::stage::{Hostname, Packages, Stage, StageResult};
use crate::stages;
// sync.rs
//
// All logic related to the `-s` sync flag should be placed here.

impl Goat {
    /// This is where 99% of the magic happens.
    ///
    /// This function is what synchronizes the system to the current configuration file. The idea is
    /// the system NEVER gets modified* unless this function is called.
    ///
    /// \*: The health check can create files and directories exclusive to `goat`'s requirements.
    pub fn sync(&self) -> anyhow::Result<()> {
        if !Uid::effective().is_root() {
            return Err(anyhow!("Sync requires root privileges!"));
        }

        // TODO: We don't want a halfway synced system so in the future we need to containerize our
        //       sync so if an error is thrown we cancel the build and have no side effects.
        
        // TODO: Macroify this later.
        // let stages = stages![ Hostname, Packages ];
        let stages: Vec<Box<dyn Stage>> = stages![
            Hostname,
            Packages
        ];
        
        for stage in stages {
            match stage.apply(self) {
                Ok(StageResult::Done) => {
                    log::warn!("Stage \"{}\" complete", stage.name())
                },
                // Let the user know a stage was skipped.
                Ok(StageResult::Skipped) => {
                    log::warn!("Skipped stage \"{}\" as it would have no effect.", stage.name())
                },
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}