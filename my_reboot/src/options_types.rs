use std::fmt::Display;

use serde::{Deserialize, Serialize};

use crate::configs::Configs;

pub trait OptionType: Copy + Eq {
    // We're assuming that all options have only two possible values.
    fn values() -> [Self; 2];

    fn to_option_string(&self) -> &str;

    fn from_option_string(option_string: &str) -> Option<Self> {
        Self::values()
            .into_iter()
            .find(|v| v.to_option_string() == option_string)
    }

    fn from_arg_string(arg_string: &str) -> Option<Self> {
        Self::from_option_string(arg_string)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum OperatingSystem {
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "linux")]
    Linux,
}
impl OptionType for OperatingSystem {
    fn values() -> [Self; 2] {
        [OperatingSystem::Windows, OperatingSystem::Linux]
    }

    fn to_option_string(&self) -> &str {
        match self {
            OperatingSystem::Windows => "windows",
            OperatingSystem::Linux => "linux",
        }
    }
}
impl Display for OperatingSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                OperatingSystem::Windows => "Windows",
                OperatingSystem::Linux => "Linux",
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub(crate) enum ProfileId {
    #[serde(rename = "a")]
    A,
    #[serde(rename = "b")]
    B,
}
impl OptionType for ProfileId {
    fn values() -> [Self; 2] {
        [Self::A, Self::B]
    }

    fn to_option_string(&self) -> &str {
        match self {
            ProfileId::A => "profile-a",
            ProfileId::B => "profile-b",
        }
    }

    fn from_arg_string(arg_string: &str) -> Option<Self> {
        match arg_string {
            "a" => Some(ProfileId::A),
            "b" => Some(ProfileId::B),
            _ => None,
        }
    }
}
impl Display for ProfileId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ProfileId::A => "A",
                ProfileId::B => "B",
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) struct LabeledProfile<'a> {
    profile_id: ProfileId,
    label: &'a str,
}
impl<'a> LabeledProfile<'a> {
    pub(crate) fn get(profile_id: ProfileId, configs: &'a Configs) -> Self {
        let label = &configs.profile[profile_id].label;
        Self::new(profile_id, label)
    }

    pub(crate) fn new(profile_id: ProfileId, label: &'a str) -> Self {
        Self { profile_id, label }
    }

    pub(crate) fn profile_id(&self) -> ProfileId {
        self.profile_id
    }
}
impl Display for LabeledProfile<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\" ({})", self.label, self.profile_id)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RebootAction {
    Reboot,
    Shutdown,
}
impl OptionType for RebootAction {
    fn values() -> [Self; 2] {
        [RebootAction::Reboot, RebootAction::Shutdown]
    }

    fn to_option_string(&self) -> &str {
        match self {
            RebootAction::Reboot => "reboot",
            RebootAction::Shutdown => "shutdown",
        }
    }
}
impl Display for RebootAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RebootAction::Reboot => "reiniciar".to_string(),
                RebootAction::Shutdown => "desligar".to_string(),
            }
        )
    }
}
