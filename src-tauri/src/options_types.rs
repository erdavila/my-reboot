pub trait OptionType: Copy {
    fn values() -> Vec<Self>;

    fn to_option_string(&self) -> &str;

    fn from_option_string(option_string: &str) -> Option<Self> {
        Self::values()
            .iter()
            .find(|v| v.to_option_string() == option_string)
            .copied()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum OperatingSystem {
    Windows,
    Linux,
}
impl OptionType for OperatingSystem {
    fn values() -> Vec<Self> {
        vec![OperatingSystem::Windows, OperatingSystem::Linux]
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
    fn values() -> Vec<Self> {
        vec![Display::Monitor, Display::TV]
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
