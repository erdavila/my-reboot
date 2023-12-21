use std::process::Command;
use std::thread;
use std::time::{Duration, Instant};

use anyhow::{bail, Result};

use crate::{configs::Configs, host_os::SuccessOr, options_types::Display, text};

use super::CurrentDisplayHandler;

mod get_active_display_id;

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

pub fn get_current_display_handler<'a>(
    configs: &'a Configs,
) -> Option<Box<dyn CurrentDisplayHandler + 'a>> {
    Some(Box::new(WindowsCurrentDisplayHandler { configs }))
}

struct WindowsCurrentDisplayHandler<'a> {
    configs: &'a Configs,
}
impl<'a> WindowsCurrentDisplayHandler<'a> {
    const DISPLAY_SWITCH_PATH: &'static str = r"C:\Windows\system32\DisplaySwitch.exe"; // TODO: is full path needed?

    fn execute_display_switch(&self, display_switch_arg: &str) -> Result<()> {
        Command::new(Self::DISPLAY_SWITCH_PATH)
            .arg(display_switch_arg)
            .status()?
            .success_or(text::display::switching::FAILED)
    }
}
impl<'a> CurrentDisplayHandler for WindowsCurrentDisplayHandler<'a> {
    fn get(&self) -> Display {
        let device_id = get_active_display_id::get_active_display_id();
        self.configs.get_display_by_device_id(&device_id)
    }

    fn switch_to(&self, display: Display) -> Result<()> {
        let display_id_before = get_active_display_id::get_active_display_id();

        let display_switch_arg = self.configs.get_display_switch_arg(display);
        self.execute_display_switch(display_switch_arg)?;

        const PROBE_INTERVAL: Duration = Duration::from_secs(1);
        const TOTAL_WAIT_TIME: Duration = Duration::from_secs(10);
        let begin = Instant::now();

        while Instant::now().duration_since(begin) < TOTAL_WAIT_TIME {
            thread::sleep(PROBE_INTERVAL);
            if get_active_display_id::get_active_display_id() != display_id_before {
                return Ok(());
            }
        }

        bail!(text::display::switching::TAKING_TOO_LONG);
    }
}
