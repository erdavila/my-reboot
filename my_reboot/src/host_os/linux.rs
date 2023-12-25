use std::process::Command;

use anyhow::Result;

use crate::options_types::{Display, OperatingSystem, RebootAction};
use crate::script::{Script, SetOrUnset};
use crate::{configs::Configs, text};

use super::{CurrentDisplayHandler, PredefinedScript, SuccessOr};

pub const STATE_DIR_PATH: &str = "/boot/grub/grubenv.dir";

pub const PREDEFINED_SCRIPTS: [PredefinedScript; 2] = [
    PredefinedScript {
        button_label: "Reiniciar no Windows usando o monitor",
        script: reboot_on_windows_with_display(Display::Monitor),
    },
    PredefinedScript {
        button_label: "Reiniciar no Windows usando a TV",
        script: reboot_on_windows_with_display(Display::TV),
    },
];

const fn reboot_on_windows_with_display(display: Display) -> Script {
    Script {
        next_boot_operating_system: Some(SetOrUnset::Set(OperatingSystem::Windows)),
        next_windows_boot_display: Some(SetOrUnset::Set(display)),
        reboot_action: Some(RebootAction::Reboot),
        ..Script::new()
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

pub fn get_current_display_handler<'a>(
    _: &'a Configs,
) -> Option<Box<dyn CurrentDisplayHandler + 'a>> {
    None
}
