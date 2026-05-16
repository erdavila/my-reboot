use std::env;

use ansi_term::{ANSIString, Color};
use anyhow::{Ok, Result};

#[cfg(windows)]
use crate::options_types::OptionType as _;
use crate::options_types::{Display, LabeledProfile, OperatingSystem, ProfileId, RebootAction};
use crate::state::StateProvider;
use crate::{host_os, text};

#[derive(Clone, Copy, Debug)]
pub struct Script {
    pub next_boot_operating_system: Option<SetOrUnset<OperatingSystem>>,
    pub(crate) next_windows_boot_profile: Option<SetOrUnset<ProfileId>>,
    pub next_windows_boot_display: Option<SetOrUnset<Display>>,
    #[cfg(windows)]
    pub(crate) switch_to_profile: Option<SwitchToProfile>,
    #[cfg(windows)]
    pub switch_to_display: Option<SwitchToDisplay>,
    pub reboot_action: Option<RebootAction>,
}
impl Script {
    pub const fn new() -> Self {
        Script {
            next_boot_operating_system: None,
            next_windows_boot_profile: None,
            next_windows_boot_display: None,
            #[cfg(windows)]
            switch_to_profile: None,
            #[cfg(windows)]
            switch_to_display: None,
            reboot_action: None,
        }
    }

    pub fn execute(self) -> Result<()> {
        let mut executor = ScriptExecutor {
            state_provider: StateProvider::new()?,
        };

        executor.execute(self)
    }
}

struct ScriptExecutor {
    state_provider: StateProvider,
}
impl ScriptExecutor {
    fn execute(&mut self, script: Script) -> Result<()> {
        if let Some(os_option) = script.next_boot_operating_system {
            self.apply_next_boot_operating_system(os_option);
        }

        if let Some(profile_option) = script.next_windows_boot_profile {
            self.apply_next_windows_boot_profile(profile_option);
        }

        if let Some(display_option) = script.next_windows_boot_display {
            self.apply_next_windows_boot_display(display_option);
        }

        #[cfg(windows)]
        if let Some(switch_to) = script.switch_to_profile {
            self.apply_switch_to_profile(switch_to)?;
        }

        #[cfg(windows)]
        if let Some(switch_to) = script.switch_to_display {
            self.apply_switch_to_display(switch_to)?;
        }

        if let Some(reboot_action) = script.reboot_action {
            Self::apply_reboot_action(reboot_action)?;
        }

        Ok(())
    }

    fn apply_next_boot_operating_system(&mut self, os_option: SetOrUnset<OperatingSystem>) {
        self.apply_option(
            &os_option,
            |sp, &os| sp.set_next_boot_operating_system(os),
            StateProvider::unset_next_boot_operating_system,
            text::operating_system::ON_NEXT_BOOT_DESCRIPTION,
            text::operating_system::WAS_UPDATED_TO,
            text::operating_system::value_text,
        );
    }

    fn apply_next_windows_boot_profile(&mut self, profile_option: SetOrUnset<ProfileId>) {
        // Store the label as an owned String to avoid capturing the state_provider lifetime.
        let profile_option = profile_option.to_option().map(|profile_id| {
            let label = self.state_provider.configs().get_profile_label(profile_id);
            (profile_id, label.to_string())
        });
        let profile_option = profile_option
            .as_ref()
            .map(|(profile_id, label)| LabeledProfile::new(*profile_id, label))
            .into();

        self.apply_option(
            &profile_option,
            |sp, lp| sp.set_next_windows_boot_profile(lp.profile_id()),
            StateProvider::unset_next_windows_boot_profile,
            text::profile::ON_NEXT_WINDOWS_BOOT_DESCRIPTION,
            text::profile::WAS_UPDATED_TO,
            text::profile::next_boot_value_text,
        );
    }

    fn apply_next_windows_boot_display(&mut self, display_option: SetOrUnset<Display>) {
        self.apply_option(
            &display_option,
            |sp, &display| sp.set_next_windows_boot_display(display),
            StateProvider::unset_next_windows_boot_display,
            text::display::ON_NEXT_WINDOWS_BOOT_DESCRIPTION,
            text::display::WAS_UPDATED_TO,
            text::display::value_text,
        );
    }

    fn apply_option<T, S, U, V>(
        &mut self,
        option: &SetOrUnset<T>,
        set: S,
        unset: U,
        description: &str,
        was_updated_to: &str,
        value_text: V,
    ) where
        T: Copy,
        S: FnOnce(&mut StateProvider, &T),
        U: FnOnce(&mut StateProvider),
        V: Fn(Option<T>) -> ANSIString<'static>,
    {
        use SetOrUnset::{Set, Unset};

        match option {
            Set(value) => set(&mut self.state_provider, value),
            Unset => unset(&mut self.state_provider),
        }

        println!(
            "{} {} {}.",
            description,
            was_updated_to,
            value_text(option.to_option())
        );
    }

    #[cfg(windows)]
    fn apply_switch_to_profile(&mut self, switch_to: SwitchToProfile) -> Result<()> {
        let from_profile = self.state_provider.get_current_profile()?;

        match switch_to {
            SwitchToProfile::Other => {
                let Some(from_profile) = from_profile else {
                    anyhow::bail!("Não foi possível identificar o perfil atual");
                };

                let to_profile = ProfileId::values()
                    .into_iter()
                    .find(|&p| p != from_profile)
                    .unwrap();

                self.switch_profile_to(to_profile)?;
            }
            SwitchToProfile::Profile(to_profile) => {
                if Some(to_profile) == from_profile {
                    let labeled_profile =
                        LabeledProfile::get(to_profile, self.state_provider.configs());
                    println!(
                        "{} {}",
                        text::profile::current_value_text(Some(labeled_profile)),
                        text::profile::switching::IS_ALREADY_CURRENT
                    );
                } else {
                    self.switch_profile_to(to_profile)?;
                }
            }
            SwitchToProfile::Saved => match self.state_provider.get_next_windows_boot_profile() {
                Some(to_profile) => {
                    if Some(to_profile) == from_profile {
                        let labeled_profile =
                            LabeledProfile::get(to_profile, self.state_provider.configs());
                        println!(
                            "O {} é {}, que já é o perfil atual",
                            text::profile::ON_NEXT_WINDOWS_BOOT_DESCRIPTION,
                            text::profile::current_value_text(Some(labeled_profile))
                        );
                    } else {
                        self.switch_profile_to(to_profile)?;
                        self.state_provider.unset_next_windows_boot_profile();
                    }
                }
                None => {
                    println!(
                        "O {} é {}",
                        text::profile::ON_NEXT_WINDOWS_BOOT_DESCRIPTION,
                        text::profile::next_boot_value_text(None)
                    );
                }
            },
        }

        Ok(())
    }

    #[cfg(windows)]
    fn switch_profile_to(&self, profile_id: ProfileId) -> Result<()> {
        let labeled_profile = LabeledProfile::get(profile_id, self.state_provider.configs());
        println!(
            "{} {}",
            text::profile::switching::TO,
            text::profile::current_value_text(Some(labeled_profile))
        );
        self.state_provider.set_current_profile(profile_id)
    }

    #[cfg(windows)]
    fn apply_switch_to_display(&mut self, switch_to: SwitchToDisplay) -> Result<()> {
        let from_display = self.state_provider.get_current_display();

        match switch_to {
            SwitchToDisplay::Other => {
                let to_display: Vec<_> = Display::values()
                    .into_iter()
                    .filter(|d| *d != from_display)
                    .collect();
                assert_eq!(to_display.len(), 1);
                let to_display = to_display[0];

                self.switch_display_to(to_display)?;
            }
            SwitchToDisplay::Display(to_display) => {
                if to_display == from_display {
                    println!(
                        "{} {}",
                        text::display::value_text(Some(to_display)),
                        text::display::switching::IS_ALREADY_CURRENT
                    );
                } else {
                    self.switch_display_to(to_display)?;
                }
            }
            SwitchToDisplay::Saved => match self.state_provider.get_next_windows_boot_display() {
                Some(to_display) => {
                    if to_display == from_display {
                        println!(
                            "A {} é {}, que já é a tela atual",
                            text::display::ON_NEXT_WINDOWS_BOOT_DESCRIPTION,
                            text::display::value_text(Some(to_display))
                        );
                    } else {
                        self.switch_display_to(to_display)?;
                        self.state_provider.unset_next_windows_boot_display();
                    }
                }
                None => println!(
                    "A {} é {}",
                    text::display::ON_NEXT_WINDOWS_BOOT_DESCRIPTION,
                    text::display::value_text(None)
                ),
            },
        }

        Ok(())
    }

    #[cfg(windows)]
    fn switch_display_to(&self, display: Display) -> Result<()> {
        println!(
            "{} {}",
            text::display::switching::TO,
            text::display::value_text(Some(display))
        );
        self.state_provider.set_current_display(display)
    }

    fn apply_reboot_action(reboot_action: RebootAction) -> Result<()> {
        match reboot_action {
            RebootAction::Reboot => Self::do_reboot_action(host_os::reboot, "Reiniciando"),
            RebootAction::Shutdown => Self::do_reboot_action(host_os::shutdown, "Desligando"),
        }
    }

    fn do_reboot_action(method: fn() -> Result<()>, message: &str) -> Result<()> {
        println!("{message}...");
        if env::var("NO_REBOOT_ACTION").is_ok() {
            println!("{} 😬", Color::Yellow.paint("...mas não de verdade!"));
            Ok(())
        } else {
            method()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SetOrUnset<T> {
    Set(T),
    Unset,
}
impl<T: Copy> SetOrUnset<T> {
    fn to_option(self) -> Option<T> {
        match self {
            SetOrUnset::Set(value) => Some(value),
            SetOrUnset::Unset => None,
        }
    }
}
impl<T> From<Option<T>> for SetOrUnset<T> {
    fn from(option: Option<T>) -> Self {
        match option {
            Some(value) => Self::Set(value),
            None => Self::Unset,
        }
    }
}

#[cfg(windows)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum SwitchToProfile {
    Other,
    Profile(ProfileId),
    Saved,
}

#[cfg(windows)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SwitchToDisplay {
    Other,
    Display(Display),
    Saved,
}
