use std::fs;
use std::path::Path;
use anyhow::anyhow;
use nix::unistd::Uid;
use crate::goat::Goat;

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

        let current_hostname = fs::read_to_string("/etc/hostname")?.trim().to_owned();
        if current_hostname != self.config.hostname {
            fs::write(Path::new("/etc/hostname"), format!("{}\n", self.config.hostname))?;

            log::warn!("Hostname changed, this will take effect next reboot. See issue #1 on github.")
            // TODO: Do some testing on changing the hostname with systemd as it tends to break 
            //       lots of things like the X server and dbus.

            //if !Command::new("sh").arg("-c").arg(&self.service_manager.hostname_reload_command).output()?.status.success() {
            //    return Err(anyhow!("Failed to run \"sh -c {}\"", self.service_manager.hostname_reload_command))
            //}
        }

        if let Some(packages) = &self.config.packages {
            let packages: Vec<&str> = packages.iter().map(|package| package.as_str()).collect();

            self.package_manager.install(packages.clone())?;
            self.package_manager.remove_unneeded_packages(packages)?;
        }

        Ok(())
    }
}