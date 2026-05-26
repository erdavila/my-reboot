use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{Result, bail};
use display_profile_lib::{Profile, SetProfileAction};

use crate::host_os::SuccessOr;
use crate::options_types::{OperatingSystem, ProfileId};
use crate::persist::configs::Configs;
use crate::text;

pub mod configuration;

pub const HOST_OS: OperatingSystem = OperatingSystem::Windows;
pub const STATE_DIR_PATH: &str = r"C:\grubenv.dir";

pub fn reboot() -> Result<()> {
    shutdown_now("/g")
}

pub fn shutdown() -> Result<()> {
    shutdown_now("/sg")
}

fn shutdown_now(arg: &str) -> Result<()> {
    Command::new("shutdown")
        .arg(arg)
        .args(["/t", "0"])
        .status()?
        .success_or(text::reboot_action::FAILED)
}

pub(crate) struct CurrentProfileHandler<'a> {
    configs: &'a Configs,
}
impl<'a> CurrentProfileHandler<'a> {
    pub(crate) fn new(configs: &'a Configs) -> Self {
        Self { configs }
    }

    fn execute_profile_switch(profile: &Profile, wait_seconds: u64) -> Result<bool> {
        const PROBE_INTERVAL: Duration = Duration::from_secs(1);

        let profile_before = display_profile_lib::get_profile()?;

        display_profile_lib::set_profile(profile, SetProfileAction::Apply)?;

        let total_wait_time: Duration = Duration::from_secs(wait_seconds);
        let begin = Instant::now();

        while Instant::now().duration_since(begin) < total_wait_time {
            thread::sleep(PROBE_INTERVAL);
            if display_profile_lib::get_profile()? != profile_before {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub(crate) fn get(&self) -> Result<Option<ProfileId>> {
        let profile = display_profile_lib::get_profile()?;
        let profile_id = self.configs.profile_id_by_config(&profile)?;
        Ok(profile_id)
    }

    pub(crate) fn switch_to(&self, profile_id: ProfileId) -> Result<()> {
        const WAIT_SECONDS: u64 = 10;

        let profile = self.configs.profile[profile_id].display_configs()?;

        let switched = Self::execute_profile_switch(&profile, WAIT_SECONDS)?;
        if switched {
            Ok(())
        } else {
            bail!(text::profile::switching::TAKING_TOO_LONG);
        }
    }
}
