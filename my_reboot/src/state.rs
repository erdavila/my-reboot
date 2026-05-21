use anyhow::Result;

use crate::configs::Configs;
use crate::grubenv::Grubenv;
#[cfg(windows)]
use crate::host_os;
use crate::options::Options;
use crate::options_types::{OperatingSystem, ProfileId};

const GRUB_ENTRY: &str = "saved_entry";

pub struct State {
    pub next_boot_operating_system: Option<OperatingSystem>,
    pub(crate) next_windows_boot_profile: Option<ProfileId>,
    #[cfg(windows)]
    pub(crate) current_profile: Option<ProfileId>,
}

pub struct StateProvider {
    grubenv: Grubenv,
    options: Options,
    configs: Configs,
}
impl StateProvider {
    pub fn new() -> Result<StateProvider> {
        let grubenv = Grubenv::load()?;
        let options = Options::load()?;
        let configs = Configs::load()?;
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
            #[cfg(windows)]
            current_profile: self.get_current_profile()?,
        })
    }

    fn get_next_boot_operating_system(&self) -> Option<OperatingSystem> {
        self.grubenv
            .get(GRUB_ENTRY)
            .map(|grub_entry| self.configs.operating_system_by_grub_entry(grub_entry))
    }

    pub fn set_next_boot_operating_system(&mut self, os: OperatingSystem) {
        let grub_entry = &self.configs.operating_system[os].grub_entry;
        self.grubenv.set(GRUB_ENTRY, grub_entry);
        self.grubenv.save().unwrap();
    }

    pub fn unset_next_boot_operating_system(&mut self) {
        self.grubenv.unset(GRUB_ENTRY);
        self.grubenv.save().unwrap();
    }

    pub(crate) fn get_next_windows_boot_profile(&self) -> Option<ProfileId> {
        self.options.operating_system.windows.profile
    }

    pub(crate) fn set_next_windows_boot_profile(&mut self, profile_id: ProfileId) {
        self.options.operating_system.windows.profile = Some(profile_id);
        self.options.save().unwrap();
    }

    pub(crate) fn unset_next_windows_boot_profile(&mut self) {
        self.options.operating_system.windows.profile = None;
        self.options.save().unwrap();
    }

    #[cfg(windows)]
    pub(crate) fn get_current_profile(&self) -> Result<Option<ProfileId>> {
        self.current_profile_handler().get()
    }

    #[cfg(windows)]
    pub(crate) fn set_current_profile(&self, profile_id: ProfileId) -> Result<()> {
        self.current_profile_handler().switch_to(profile_id)
    }

    #[cfg(windows)]
    fn current_profile_handler(&self) -> host_os::CurrentProfileHandler<'_> {
        host_os::CurrentProfileHandler::new(&self.configs)
    }

    pub(crate) fn configs(&self) -> &Configs {
        &self.configs
    }
}
