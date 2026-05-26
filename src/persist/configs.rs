use std::ops::Index;
use std::path::PathBuf;
use std::{fs, io};

use anyhow::{Context, Result};
#[cfg(windows)]
use display_profile_lib::Profile;
use serde::{Deserialize, Serialize};

use crate::host_os::state_path;
use crate::options_types::{OperatingSystem, OptionType, ProfileId};

const CONFIGS_FILENAME: &str = "my-reboot-configs.toml";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct Configs {
    pub(crate) operating_system: OperatingSystemsConfigs,
    pub(crate) profile: ProfilesConfigs,
}
impl Configs {
    pub(crate) fn load() -> Result<Configs> {
        match fs::read_to_string(Self::path()) {
            Ok(content) => {
                let configs =
                    Self::from_serialized(&content).with_context(|| "O conteúdo do arquivo de configurações está incompleto ou é inválido.")?;
                Ok(configs)
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                Err(e).context("Arquivo de configurações não encontrado. Execute 'my-reboot configure' no Windows e no Linux para criar o arquivo com todo o conteúdo necessário.")
            },
            Err(e) => Err(e.into()),
        }
    }

    fn from_serialized(serialized: &str) -> Result<Self> {
        let configs = toml::from_str(serialized)?;
        Ok(configs)
    }

    pub(crate) fn operating_system_by_grub_entry(&self, grub_entry: &str) -> OperatingSystem {
        OperatingSystem::values()
            .into_iter()
            .find(|os| self.operating_system[*os].grub_entry == grub_entry)
            .unwrap()
    }

    #[cfg(windows)]
    pub(crate) fn profile_id_by_config(&self, profile: &Profile) -> Result<Option<ProfileId>> {
        for id in ProfileId::values() {
            let config: Profile = serde_json::from_str(&self.profile[id].display_configs)?;
            if config == *profile {
                return Ok(Some(id));
            }
        }

        Ok(None)
    }

    fn path() -> PathBuf {
        state_path(CONFIGS_FILENAME)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct OperatingSystemsConfigs {
    windows: OperatingSystemConfigs,
    linux: OperatingSystemConfigs,
}
impl Index<OperatingSystem> for OperatingSystemsConfigs {
    type Output = OperatingSystemConfigs;

    fn index(&self, index: OperatingSystem) -> &Self::Output {
        match index {
            OperatingSystem::Windows => &self.windows,
            OperatingSystem::Linux => &self.linux,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct OperatingSystemConfigs {
    pub(crate) grub_entry: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ProfilesConfigs {
    pub(crate) a: ProfileConfigs,
    pub(crate) b: ProfileConfigs,
}
impl Index<ProfileId> for ProfilesConfigs {
    type Output = ProfileConfigs;

    fn index(&self, index: ProfileId) -> &Self::Output {
        match index {
            ProfileId::A => &self.a,
            ProfileId::B => &self.b,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct ProfileConfigs {
    pub(crate) label: String,
    pub(crate) display_configs: String,
}
#[cfg(windows)]
impl ProfileConfigs {
    pub(crate) fn display_configs(&self) -> Result<Profile> {
        let profile = serde_json::from_str(&self.display_configs)?;
        Ok(profile)
    }
}

const OPERATING_SYSTEM_KEY: &str = "operating_system";
const PROFILE_KEY: &str = "profile";

pub(crate) struct ConfigsWriter {
    content: toml::Table,
}
impl ConfigsWriter {
    pub(crate) fn load() -> Result<ConfigsWriter> {
        match fs::read_to_string(Configs::path()) {
            Ok(content) => {
                let content: toml::Table = toml::from_str(&content)?;
                Ok(ConfigsWriter { content })
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(ConfigsWriter {
                content: toml::Table::new(),
            }),
            Err(e) => Err(e.into()),
        }
    }

    #[cfg(any(not(windows), test))]
    pub(crate) fn set_grub_entry(&mut self, os: OperatingSystem, grub_entry: &str) -> Result<()> {
        self.add_to_table(
            OPERATING_SYSTEM_KEY,
            os,
            OperatingSystemConfigs {
                grub_entry: grub_entry.to_string(),
            },
        )
    }

    pub(crate) fn has_grub_entry(&self, os: OperatingSystem) -> bool {
        self.table_has::<OperatingSystemConfigs>(OPERATING_SYSTEM_KEY, os)
    }

    #[cfg(windows)]
    pub(crate) fn set_profile_configs(
        &mut self,
        id: ProfileId,
        label: &str,
        display_configs: &Profile,
    ) -> Result<()> {
        self.set_profile_configs_strs(id, label, &serde_json::to_string(display_configs)?)
    }

    #[cfg(any(windows, test))]
    fn set_profile_configs_strs(
        &mut self,
        id: ProfileId,
        label: &str,
        configs: &str,
    ) -> Result<()> {
        self.add_to_table(
            PROFILE_KEY,
            id,
            ProfileConfigs {
                label: label.to_string(),
                display_configs: configs.to_string(),
            },
        )
    }

    pub(crate) fn has_profile_configs(&self, id: ProfileId) -> bool {
        self.table_has::<ProfileConfigs>(PROFILE_KEY, id)
    }

    fn add_to_table<K, V>(&mut self, table_key: &str, key: K, value: V) -> Result<()>
    where
        K: Serialize,
        V: Serialize,
    {
        let key = toml::Value::try_from(key)?.as_str().unwrap().to_string();
        let value = toml::Value::try_from(value)?;

        self.content
            .entry(table_key)
            .or_insert_with(|| toml::Table::new().into())
            .as_table_mut()
            .unwrap()
            .insert(key, value);

        Ok(())
    }

    fn table_has<V>(&self, table_key: &str, key: impl Serialize) -> bool
    where
        V: for<'a> Deserialize<'a>,
    {
        let key = toml::Value::try_from(key).unwrap();
        let key = key.as_str().unwrap();

        let get_value = || {
            self.content
                .get(table_key)?
                .as_table()?
                .get(key)?
                .clone()
                .try_into::<V>()
                .ok()
        };

        get_value().is_some()
    }

    pub(crate) fn save(&self) -> Result<()> {
        fs::write(Configs::path(), self.serialized()?)?;
        Ok(())
    }

    fn serialized(&self) -> Result<String> {
        let content = toml::to_string(&self.content)?;
        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_written_by_the_writer_can_be_read_by_the_reader() -> Result<()> {
        let mut writer = ConfigsWriter {
            content: toml::Table::new(),
        };

        writer.set_grub_entry(OperatingSystem::Windows, "windows-grub-entry")?;
        writer.set_grub_entry(OperatingSystem::Linux, "linux-grub-entry")?;
        writer.set_profile_configs_strs(
            ProfileId::A,
            "profile-a-label",
            "profile-a-display-configs",
        )?;
        writer.set_profile_configs_strs(
            ProfileId::B,
            "profile-b-label",
            "profile-b-display-configs",
        )?;

        let serialized = writer.serialized()?;
        let configs = Configs::from_serialized(&serialized)?;

        assert_eq!(
            configs,
            Configs {
                operating_system: OperatingSystemsConfigs {
                    windows: OperatingSystemConfigs {
                        grub_entry: "windows-grub-entry".to_string()
                    },
                    linux: OperatingSystemConfigs {
                        grub_entry: "linux-grub-entry".to_string()
                    },
                },
                profile: ProfilesConfigs {
                    a: ProfileConfigs {
                        label: "profile-a-label".to_string(),
                        display_configs: "profile-a-display-configs".to_string(),
                    },
                    b: ProfileConfigs {
                        label: "profile-b-label".to_string(),
                        display_configs: "profile-b-display-configs".to_string(),
                    },
                }
            }
        );

        Ok(())
    }

    #[test]
    fn writer_set_and_get_grub_entry() -> Result<()> {
        let mut writer = ConfigsWriter {
            content: toml::Table::new(),
        };
        assert!(!writer.has_grub_entry(OperatingSystem::Windows));
        assert!(!writer.has_grub_entry(OperatingSystem::Linux));

        writer.set_grub_entry(OperatingSystem::Windows, "windows-grub-entry")?;
        assert!(writer.has_grub_entry(OperatingSystem::Windows));
        assert!(!writer.has_grub_entry(OperatingSystem::Linux));

        writer.set_grub_entry(OperatingSystem::Linux, "linux-grub-entry")?;
        assert!(writer.has_grub_entry(OperatingSystem::Windows));
        assert!(writer.has_grub_entry(OperatingSystem::Linux));

        Ok(())
    }

    #[test]
    fn writer_set_and_has_profile_configs() -> Result<()> {
        let mut writer = ConfigsWriter {
            content: toml::Table::new(),
        };
        assert!(!writer.has_profile_configs(ProfileId::A));
        assert!(!writer.has_profile_configs(ProfileId::B));

        writer.set_profile_configs_strs(
            ProfileId::A,
            "profile-a-label",
            "profile-a-display-configs",
        )?;
        assert!(writer.has_profile_configs(ProfileId::A));
        assert!(!writer.has_profile_configs(ProfileId::B));

        writer.set_profile_configs_strs(
            ProfileId::B,
            "profile-b-label",
            "profile-b-display-configs",
        )?;
        assert!(writer.has_profile_configs(ProfileId::A));
        assert!(writer.has_profile_configs(ProfileId::B));

        Ok(())
    }
}
