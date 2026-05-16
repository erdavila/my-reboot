use std::io;

use anyhow::Result;

use crate::configs::Configs;
use crate::grubenv::Grubenv;
#[cfg(windows)]
use crate::host_os;
use crate::options_types::{Display, OperatingSystem, OptionType as _, ProfileId};
use crate::properties::Properties;

const GRUB_ENTRY: &str = "saved_entry";
const WINDOWS_PROFILE_KEY: &str = "windows.profile";
const WINDOWS_DISPLAY_KEY: &str = "windows.display";
const OPTIONS_FILENAME: &str = "my-reboot-options.properties";

pub struct State {
    pub next_boot_operating_system: Option<OperatingSystem>,
    pub(crate) next_windows_boot_profile: Option<ProfileId>,
    pub next_windows_boot_display: Option<Display>,
    #[cfg(windows)]
    pub(crate) current_profile: Option<ProfileId>,
    #[cfg(windows)]
    pub current_display: Display,
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

    #[cfg_attr(not(windows), expect(clippy::unnecessary_wraps))]
    pub fn get_state(&self) -> Result<State> {
        Ok(State {
            next_boot_operating_system: self.get_next_boot_operating_system(),
            next_windows_boot_profile: self.get_next_windows_boot_profile(),
            next_windows_boot_display: self.get_next_windows_boot_display(),
            #[cfg(windows)]
            current_profile: self.get_current_profile()?,
            #[cfg(windows)]
            current_display: self.get_current_display(),
        })
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

    pub(crate) fn get_next_windows_boot_profile(&self) -> Option<ProfileId> {
        self.options
            .get(WINDOWS_PROFILE_KEY)
            .and_then(|code| ProfileId::from_option_string(code))
    }

    pub(crate) fn set_next_windows_boot_profile(&mut self, profile_id: ProfileId) {
        self.options
            .set(WINDOWS_PROFILE_KEY, profile_id.to_option_string());
        self.options.save().unwrap();
    }

    pub(crate) fn unset_next_windows_boot_profile(&mut self) {
        self.options.unset(WINDOWS_PROFILE_KEY);
        self.options.save().unwrap();
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

    #[cfg(windows)]
    pub(crate) fn get_current_profile(&self) -> Result<Option<ProfileId>> {
        self.current_profile_handler().get()
    }

    #[cfg(windows)]
    pub fn get_current_display(&self) -> Display {
        self.current_display_handler().get()
    }

    #[cfg(windows)]
    pub fn set_current_display(&self, display: Display) -> Result<()> {
        self.current_display_handler().switch_to(display)
    }

    #[cfg(windows)]
    fn current_profile_handler(&self) -> host_os::CurrentProfileHandler<'_> {
        host_os::CurrentProfileHandler::new(&self.configs)
    }

    #[cfg(windows)]
    fn current_display_handler(&self) -> host_os::CurrentDisplayHandler<'_> {
        host_os::CurrentDisplayHandler::new(&self.configs)
    }

    pub(crate) fn configs(&self) -> &Configs {
        &self.configs
    }
}
