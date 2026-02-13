use std::io;

use anyhow::{Result, anyhow};

use crate::configs::Configs;
use crate::grubenv::Grubenv;
use crate::host_os;
use crate::options_types::{Display, OperatingSystem, OptionType};
use crate::properties::Properties;
use crate::text;

const GRUB_ENTRY: &str = "saved_entry";
const WINDOWS_DISPLAY_KEY: &str = "windows.display";
const OPTIONS_FILENAME: &str = "my-reboot-options.properties";

pub struct State {
    pub next_boot_operating_system: Option<OperatingSystem>,
    pub next_windows_boot_display: Option<Display>,
    pub current_display: Option<Display>,
}

pub struct StateProvider {
    grubenv: Grubenv,
    options: Properties,
    configs: Configs,
}
impl StateProvider {
    pub fn new() -> io::Result<StateProvider> {
        let grubenv = Grubenv::load()?;
        let options = Properties::load(OPTIONS_FILENAME, false)?;
        let configs = Configs::load(true)?;
        Ok(StateProvider {
            grubenv,
            options,
            configs,
        })
    }

    pub fn get_state(&self) -> State {
        State {
            next_boot_operating_system: self.get_next_boot_operating_system(),
            next_windows_boot_display: self.get_next_windows_boot_display(),
            current_display: self.get_current_display(),
        }
    }

    fn get_next_boot_operating_system(&self) -> Option<OperatingSystem> {
        self.grubenv
            .get(GRUB_ENTRY)
            .map(|grub_entry| self.configs.get_operating_system_by_grub_entry(grub_entry))
    }

    pub fn set_next_boot_operating_system(&mut self, os: OperatingSystem) {
        let grub_entry = self.configs.get_grub_entry(os);
        self.grubenv.set(GRUB_ENTRY, &grub_entry);
        self.grubenv.save().unwrap();
    }

    pub fn unset_next_boot_operating_system(&mut self) {
        self.grubenv.unset(GRUB_ENTRY);
        self.grubenv.save().unwrap();
    }

    pub fn get_next_windows_boot_display(&self) -> Option<Display> {
        self.options
            .get(WINDOWS_DISPLAY_KEY)
            .and_then(|code| Display::from_option_string(code))
    }

    pub fn set_next_windows_boot_display(&mut self, display: Display) {
        self.options
            .set(WINDOWS_DISPLAY_KEY, display.to_option_string());
        self.options.save().unwrap();
    }

    pub fn unset_next_windows_boot_display(&mut self) {
        self.options.unset(WINDOWS_DISPLAY_KEY);
        self.options.save().unwrap();
    }

    pub fn get_current_display(&self) -> Option<Display> {
        host_os::get_current_display_handler(&self.configs).map(|handler| handler.get())
    }

    pub fn set_current_display(&self, display: Display) -> Result<()> {
        let handler = host_os::get_current_display_handler(&self.configs)
            .ok_or_else(|| anyhow!(text::display::switching::NOT_SUPPORTED))?;
        handler.switch_to(display)?;
        Ok(())
    }
}
