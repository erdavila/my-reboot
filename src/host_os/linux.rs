use std::process::Command;

use anyhow::Result;

use super::{PredefinedScript, SuccessOr};
use crate::options_types::{OperatingSystem, ProfileId, RebootAction};
use crate::script::{Script, SetOrUnset};
use crate::text;

pub mod configuration;

pub const HOST_OS: OperatingSystem = OperatingSystem::Linux;
pub const STATE_DIR_PATH: &str = "/boot/grub/grubenv.dir";

pub const PREDEFINED_SCRIPTS: [PredefinedScript; 2] = [
    PredefinedScript {
        button_label_template: "{reboot_action} no {next_boot_operating_system} usando o perfil {next_windows_boot_profile}",
        script: reboot_on_windows_with_profile(ProfileId::A),
    },
    PredefinedScript {
        button_label_template: "{reboot_action} no {next_boot_operating_system} usando o perfil {next_windows_boot_profile}",
        script: reboot_on_windows_with_profile(ProfileId::B),
    },
];

const fn reboot_on_windows_with_profile(profile_id: ProfileId) -> Script {
    Script {
        next_boot_operating_system: Some(SetOrUnset::Set(OperatingSystem::Windows)),
        next_windows_boot_profile: Some(SetOrUnset::Set(profile_id)),
        #[cfg(test)]
        switch_to_profile: None,
        reboot_action: Some(RebootAction::Reboot),
    }
}

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
