use std::process::Command;

use anyhow::Result;

use super::SuccessOr;
use crate::options_types::OperatingSystem;
use crate::text;

pub mod configuration;

pub const HOST_OS: OperatingSystem = OperatingSystem::Linux;
pub(super) const DEFAULT_STATE_DIR_PATH: &str = "/boot/grub/grubenv.dir";

pub fn reboot() -> Result<()> {
    systemctl("reboot")
}

pub fn shutdown() -> Result<()> {
    systemctl("poweroff")
}

fn systemctl(arg: &str) -> Result<()> {
    Command::new("systemctl")
        .arg(arg)
        .status()?
        .success_or(text::reboot_action::FAILED)
}
