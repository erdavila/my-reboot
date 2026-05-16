pub trait OptionType: Copy + Eq {
    // We're assuming that all options have only two possible values.
    fn values() -> [Self; 2];

    fn to_option_string(&self) -> &str;

    fn from_option_string(option_string: &str) -> Option<Self> {
        Self::values()
            .into_iter()
            .find(|v| v.to_option_string() == option_string)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum OperatingSystem {
    Windows,
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
impl std::fmt::Display for OperatingSystem {
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Display {
    Monitor,
    TV,
}
impl OptionType for Display {
    fn values() -> [Self; 2] {
        [Display::Monitor, Display::TV]
    }

    fn to_option_string(&self) -> &str {
        match self {
            Display::Monitor => "monitor",
            Display::TV => "tv",
        }
    }
}
impl std::fmt::Display for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Display::Monitor => "monitor",
                Display::TV => "TV",
            }
        )
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
