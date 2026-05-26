use std::env;
use std::fmt::Debug;

use ansi_term::{ANSIString, Color};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[cfg(windows)]
use crate::options_types::Values as _;
use crate::options_types::{LabeledProfile, OperatingSystem, ProfileId, RebootAction};
use crate::state::StateProvider;
use crate::{host_os, text};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Script {
    pub next_boot_operating_system: Option<SetOrUnset<OperatingSystem>>,
    pub(crate) next_windows_boot_profile: Option<SetOrUnset<ProfileId>>,
    #[cfg(any(windows, test))]
    pub(crate) switch_to_profile: Option<SwitchToProfile>,
    pub reboot_action: Option<RebootAction>,
}
impl Script {
    pub const fn new() -> Self {
        Script {
            next_boot_operating_system: None,
            next_windows_boot_profile: None,
            #[cfg(any(windows, test))]
            switch_to_profile: None,
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

        #[cfg(windows)]
        if let Some(switch_to) = script.switch_to_profile {
            self.apply_switch_to_profile(switch_to)?;
        }

        if let Some(reboot_action) = script.reboot_action {
            Self::apply_reboot_action(reboot_action)?;
        }

        Ok(())
    }

    fn apply_next_boot_operating_system(&mut self, os_option: SetOrUnset<OperatingSystem>) {
        self.apply_option(
            os_option,
            StateProvider::set_next_boot_operating_system,
            std::convert::identity,
            text::operating_system::ON_NEXT_BOOT_DESCRIPTION,
            text::operating_system::WAS_UPDATED_TO,
            text::operating_system::value_text,
        );
    }

    fn apply_next_windows_boot_profile(&mut self, profile_option: SetOrUnset<ProfileId>) {
        // Clone the label to avoid capturing the state_provider lifetime.
        let profile_option = profile_option.into_option().map(|profile_id| {
            let label = self.state_provider.configs().profile[profile_id]
                .label
                .clone();
            (profile_id, label)
        });
        let profile_option = profile_option
            .as_ref()
            .map(|(profile_id, label)| LabeledProfile::new(*profile_id, label))
            .into();

        self.apply_option(
            profile_option,
            StateProvider::set_next_windows_boot_profile,
            LabeledProfile::profile_id,
            text::profile::ON_NEXT_WINDOWS_BOOT_DESCRIPTION,
            text::profile::WAS_UPDATED_TO,
            text::profile::next_boot_value_text,
        );
    }

    fn apply_option<T: Copy, U>(
        &mut self,
        option: SetOrUnset<T>,
        set: impl FnOnce(&mut StateProvider, Option<U>),
        extract: impl FnOnce(T) -> U,
        description: &str,
        was_updated_to: &str,
        value_text: impl FnOnce(Option<T>) -> ANSIString<'static>,
    ) {
        match option {
            SetOrUnset::Set(option) => set(&mut self.state_provider, Some(extract(option))),
            SetOrUnset::Unset => set(&mut self.state_provider, None),
        }

        println!(
            "{} {} {}.",
            description,
            was_updated_to,
            value_text(option.into_option())
        );
    }

    #[cfg(windows)]
    fn apply_switch_to_profile(&mut self, switch_to: SwitchToProfile) -> Result<()> {
        let from_profile = self.state_provider.current_profile()?;

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
            SwitchToProfile::Saved => match self.state_provider.next_windows_boot_profile() {
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
                        self.state_provider.next_windows_boot_profile();
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

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum SetOrUnset<T> {
    #[serde(rename = "unset")]
    Unset,
    #[serde(untagged)]
    Set(T),
}
impl<T> SetOrUnset<T> {
    pub(crate) fn into_option(self) -> Option<T> {
        match self {
            SetOrUnset::Set(value) => Some(value),
            SetOrUnset::Unset => None,
        }
    }
}
impl<T> From<T> for SetOrUnset<T> {
    fn from(value: T) -> Self {
        SetOrUnset::Set(value)
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

#[cfg(any(windows, test))]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub(crate) enum SwitchToProfile {
    #[serde(rename = "other")]
    Other,
    #[serde(rename = "saved")]
    Saved,
    #[serde(untagged)]
    Profile(ProfileId),
}
#[cfg(windows)]
impl From<ProfileId> for SwitchToProfile {
    fn from(value: ProfileId) -> Self {
        SwitchToProfile::Profile(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::options_types::{DeserializeFromString as _, SerializeToString as _};

    #[test]
    fn set_or_unset_operating_system_serialize_to_string() {
        let cases = [
            (SetOrUnset::Set(OperatingSystem::Windows), "windows"),
            (SetOrUnset::Set(OperatingSystem::Linux), "linux"),
            (SetOrUnset::Unset, "unset"),
        ];

        for (value, expected) in cases {
            assert_eq!(value.serialize_to_string(), expected);
        }
    }

    #[test]
    fn set_or_unset_operating_system_deserialize_from_string() {
        let cases = [
            ("windows", Some(SetOrUnset::Set(OperatingSystem::Windows))),
            ("linux", Some(SetOrUnset::Set(OperatingSystem::Linux))),
            ("unset", Some(SetOrUnset::Unset)),
            ("invalid", None),
        ];

        for (s, expected) in cases {
            assert_eq!(
                SetOrUnset::<OperatingSystem>::deserialize_from_string(s),
                expected
            );
        }
    }

    #[test]
    fn set_or_unset_profile_id_serialize_to_string() {
        let cases = [
            (SetOrUnset::Set(ProfileId::A), "a"),
            (SetOrUnset::Set(ProfileId::B), "b"),
            (SetOrUnset::Unset, "unset"),
        ];

        for (value, expected) in cases {
            assert_eq!(value.serialize_to_string(), expected);
        }
    }

    #[test]
    fn set_or_unset_profile_id_deserialize_from_string() {
        let cases = [
            ("a", Some(SetOrUnset::Set(ProfileId::A))),
            ("b", Some(SetOrUnset::Set(ProfileId::B))),
            ("unset", Some(SetOrUnset::Unset)),
            ("invalid", None),
        ];

        for (s, expected) in cases {
            assert_eq!(
                SetOrUnset::<ProfileId>::deserialize_from_string(s),
                expected
            );
        }
    }

    #[test]
    fn switch_to_profile_serialize_to_string() {
        let cases = [
            (SwitchToProfile::Other, "other"),
            (SwitchToProfile::Saved, "saved"),
            (SwitchToProfile::Profile(ProfileId::A), "a"),
            (SwitchToProfile::Profile(ProfileId::B), "b"),
        ];

        for (value, expected) in cases {
            assert_eq!(value.serialize_to_string(), expected);
        }
    }

    #[test]
    fn switch_to_profile_deserialize_from_string() {
        let cases = [
            ("other", Some(SwitchToProfile::Other)),
            ("saved", Some(SwitchToProfile::Saved)),
            ("a", Some(SwitchToProfile::Profile(ProfileId::A))),
            ("b", Some(SwitchToProfile::Profile(ProfileId::B))),
            ("invalid", None),
        ];

        for (s, expected) in cases {
            assert_eq!(SwitchToProfile::deserialize_from_string(s), expected);
        }
    }
}
