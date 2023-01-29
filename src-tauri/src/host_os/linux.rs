use std::process::Command;

use anyhow::Result;

use crate::{configs::Configs, text};

use super::{CurrentDisplayHandler, SuccessOr};

pub const STATE_DIR_PATH: &str = "/boot/grub/grubenv.dir";

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

pub fn get_current_display_handler<'a>(
    _: &'a Configs,
) -> Option<Box<dyn CurrentDisplayHandler + 'a>> {
    None
}
