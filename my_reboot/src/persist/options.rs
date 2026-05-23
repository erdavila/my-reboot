use std::path::PathBuf;
use std::{fs, io};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::host_os::state_path;
use crate::options_types::ProfileId;

const OPTIONS_FILENAME: &str = "my-reboot-options.toml";

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub(crate) struct Options {
    pub(crate) operating_system: OperatingSystemsOptions,
}
impl Options {
    pub(crate) fn load() -> Result<Self> {
        match fs::read_to_string(Self::path()) {
            Ok(content) => {
                let options = toml::from_str(&content)?;
                Ok(options)
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(Options::default()),
            Err(e) => Err(e.into()),
        }
    }

    pub(crate) fn save(&self) -> Result<()> {
        fs::write(Self::path(), toml::to_string(self)?)?;
        Ok(())
    }

    fn path() -> PathBuf {
        state_path(OPTIONS_FILENAME)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub(crate) struct OperatingSystemsOptions {
    pub(crate) windows: OperatingSystemOptions,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub(crate) struct OperatingSystemOptions {
    pub(crate) profile: Option<ProfileId>,
}
