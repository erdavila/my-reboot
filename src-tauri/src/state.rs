use std::io;

use crate::configs::Configs;
use crate::grubenv::Grubenv;
use crate::host_os;
use crate::options_types::{Display, OperatingSystem, OptionType};
use crate::properties::Properties;

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
        let options = Properties::load(OPTIONS_FILENAME, true)?;
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
        self.grubenv.set(GRUB_ENTRY, grub_entry);
        self.grubenv.save().unwrap();
    }

    pub fn unset_next_boot_operating_system(&mut self) {
        self.grubenv.unset(GRUB_ENTRY);
        self.grubenv.save().unwrap();
    }

    fn get_next_windows_boot_display(&self) -> Option<Display> {
        self.options
            .get(WINDOWS_DISPLAY_KEY)
            .and_then(|code| Display::from_option_string(code))
    }

    fn get_current_display(&self) -> Option<Display> {
        host_os::get_active_display_id().map(|id| self.configs.get_display_by_device_id(&id))
    }
}
