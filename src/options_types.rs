use std::fmt::{Debug, Display};

use serde::de::value::StrDeserializer;
use serde::{Deserialize, Serialize};

use crate::persist::configs::Configs;

pub(crate) trait Values: Copy {
    // We're assuming that all options have only two possible values.
    fn values() -> [Self; 2];
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum OperatingSystem {
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "linux")]
    Linux,
}
impl Values for OperatingSystem {
    fn values() -> [Self; 2] {
        [OperatingSystem::Windows, OperatingSystem::Linux]
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
impl Values for ProfileId {
    fn values() -> [Self; 2] {
        [Self::A, Self::B]
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

    pub(crate) fn profile_id(self) -> ProfileId {
        self.profile_id
    }
}
impl Display for LabeledProfile<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\" ({})", self.label, self.profile_id)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum RebootAction {
    #[serde(rename = "reboot")]
    Reboot,
    #[serde(rename = "shutdown")]
    Shutdown,
}
impl Values for RebootAction {
    fn values() -> [Self; 2] {
        [RebootAction::Reboot, RebootAction::Shutdown]
    }
}
impl Display for RebootAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RebootAction::Reboot => "reiniciar",
                RebootAction::Shutdown => "desligar",
            }
        )
    }
}

pub(crate) trait SerializeToString {
    fn serialize_to_string(&self) -> String;
}
impl<T: Serialize> SerializeToString for T {
    fn serialize_to_string(&self) -> String {
        std::fmt::from_fn(|f| self.serialize(f)).to_string()
    }
}

pub(crate) trait DeserializeFromString: Sized {
    fn deserialize_from_string(s: &str) -> Option<Self>;
}
impl<'de, T: Deserialize<'de>> DeserializeFromString for T {
    fn deserialize_from_string(s: &str) -> Option<Self> {
        let deserializer = StrDeserializer::<serde::de::value::Error>::new(s);
        T::deserialize(deserializer).ok()
    }
}
