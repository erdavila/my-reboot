use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{bail, Result};

use crate::options_types::{OperatingSystem, RebootAction};
use crate::script::{Script, SetOrUnset};
use crate::{configs::Configs, host_os::SuccessOr, options_types::Display, text};

use super::{CurrentDisplayHandler, PredefinedScript};

pub mod configuration;
mod get_active_display_id;

pub const STATE_DIR_PATH: &str = r"C:\grubenv.dir";

pub const PREDEFINED_SCRIPTS: [PredefinedScript; 1] = [PredefinedScript {
    button_label: "Reiniciar no Linux",
    script: Script {
        next_boot_operating_system: Some(SetOrUnset::Set(OperatingSystem::Linux)),
        reboot_action: Some(RebootAction::Reboot),
        ..Script::new()
    },
}];

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

pub fn get_current_display_handler<'a>(
    configs: &'a Configs,
) -> Option<Box<dyn CurrentDisplayHandler + 'a>> {
    Some(Box::new(WindowsCurrentDisplayHandler { configs }))
}

pub struct WindowsCurrentDisplayHandler<'a> {
    configs: &'a Configs,
}
impl<'a> WindowsCurrentDisplayHandler<'a> {
    const DISPLAY_SWITCH_PATH: &'static str = "DisplaySwitch.exe";

    fn execute_display_switch(display_switch_arg: &str, wait_seconds: u64) -> Result<bool> {
        let display_id_before = get_active_display_id::get_active_display_id();

        Command::new(Self::DISPLAY_SWITCH_PATH)
            .arg(display_switch_arg)
            .status()?
            .success_or(text::display::switching::FAILED)?;

        const PROBE_INTERVAL: Duration = Duration::from_secs(1);
        let total_wait_time: Duration = Duration::from_secs(wait_seconds);
        let begin = Instant::now();

        while Instant::now().duration_since(begin) < total_wait_time {
            thread::sleep(PROBE_INTERVAL);
            if get_active_display_id::get_active_display_id() != display_id_before {
                return Ok(true);
            }
        }

        Ok(false)
    }
}
impl<'a> CurrentDisplayHandler for WindowsCurrentDisplayHandler<'a> {
    fn get(&self) -> Display {
        let device_id = get_active_display_id::get_active_display_id();
        self.configs.get_display_by_device_id(&device_id)
    }

    fn switch_to(&self, display: Display) -> Result<()> {
        const WAIT_SECONDS: u64 = 10;

        let display_switch_arg = self.configs.get_display_switch_arg(display);
        let switched = Self::execute_display_switch(&display_switch_arg, WAIT_SECONDS)?;
        if switched {
            Ok(())
        } else {
            bail!(text::display::switching::TAKING_TOO_LONG);
        }
    }
}
