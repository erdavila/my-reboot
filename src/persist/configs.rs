use std::ops::Index;
use std::path::PathBuf;
use std::{fs, io};

use anyhow::{Context, Result};
#[cfg(windows)]
use display_profile_lib::Profile;
use serde::{Deserialize, Serialize};

use crate::host_os::{TemplateResolver, state_path};
use crate::options_types::{
    LabeledProfile, OperatingSystem, ProfileId, RebootAction, SerializeToString, Values as _,
};
use crate::script::{Script, SetOrUnset};
use crate::text::{self, Capitalized};

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
    pub(crate) scripts: Vec<PredefinedScript>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct PredefinedScript {
    #[serde(flatten)]
    pub(crate) script: Script,
    pub(crate) label_template: String,
}
impl PredefinedScript {
    pub(crate) fn resolve_label(&self, configs: &Configs) -> String {
        let profile_label = |profile_id| LabeledProfile::get(profile_id, configs).to_string();

        let mut template_resolver = TemplateResolver::new(&self.label_template);

        template_resolver.resolve_set_or_unset_option(
            "next_boot_operating_system",
            self.script.next_boot_operating_system,
            text::operating_system::UNDEFINED,
        );
        template_resolver.resolve_set_or_unset_option_with(
            "next_windows_boot_profile",
            self.script.next_windows_boot_profile,
            profile_label,
            text::profile::UNDEFINED,
        );
        #[cfg(windows)]
        template_resolver.resolve_option_with(
            "switch_to_profile",
            self.script.switch_to_profile,
            |switch_to| {
                use crate::script::SwitchToProfile;
                match switch_to {
                    SwitchToProfile::Other => "outro".to_string(),
                    SwitchToProfile::Profile(profile_id) => profile_label(profile_id),
                    SwitchToProfile::Saved => "salvo".to_string(),
                }
            },
            text::profile::UNDEFINED,
        );
        template_resolver.resolve_option(
            "reboot_action",
            self.script.reboot_action,
            text::reboot_action::UNDEFINED,
        );

        Capitalized(template_resolver.into_label()).to_string()
    }
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
const GRUB_ENTRY_KEY: &str = "grub_entry";
const SCRIPTS_KEY: &str = "scripts";

pub(crate) struct ConfigsWriter {
    content: Content,
}
impl ConfigsWriter {
    pub(crate) fn load() -> Result<ConfigsWriter> {
        match fs::read_to_string(Configs::path()) {
            Ok(content) => {
                let content: toml::Table = toml::from_str(&content)?;
                let content = Content::from(content);
                Ok(ConfigsWriter { content })
            }
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                let content = Content::default();
                Ok(ConfigsWriter { content })
            }
            Err(e) => Err(e.into()),
        }
    }

    #[cfg(any(not(windows), test))]
    pub(crate) fn set_grub_entry(&mut self, os: OperatingSystem, grub_entry: &str) {
        self.content
            .ensure_operating_system_configs_table(os)
            .insert(GRUB_ENTRY_KEY.to_string(), grub_entry.into());
    }

    pub(crate) fn has_grub_entry(&self, os: OperatingSystem) -> bool {
        self.content
            .operating_system_configs_table(os)
            .is_some_and(|os_configs| os_configs.contains_key(GRUB_ENTRY_KEY))
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
        let profile_table = self.content.ensure_profile_configs_table(id);
        *profile_table = toml::Table::try_from(ProfileConfigs {
            label: label.to_string(),
            display_configs: configs.to_string(),
        })?;

        Ok(())
    }

    pub(crate) fn has_profile_configs(&self, id: ProfileId) -> bool {
        self.content
            .profile_configs_table(id)
            .is_some_and(|profile_configs| {
                profile_configs.clone().try_into::<ProfileConfigs>().is_ok()
            })
    }

    pub(crate) fn save(&self) -> Result<()> {
        fs::write(Configs::path(), self.serialized()?)?;
        Ok(())
    }

    fn serialized(&self) -> Result<String> {
        let content = toml::to_string(&self.content.0)?;
        Ok(content)
    }
}

struct Content(toml::Table);
impl Content {
    fn operating_system_configs_table(&self, os: OperatingSystem) -> Option<&toml::Table> {
        self.0.table_at(OPERATING_SYSTEM_KEY)?.table_at(os)
    }

    fn ensure_operating_system_configs_table(&mut self, os: OperatingSystem) -> &mut toml::Table {
        self.0
            .ensure_table_at(OPERATING_SYSTEM_KEY)
            .ensure_table_at(os)
    }

    fn profile_configs_table(&self, id: ProfileId) -> Option<&toml::Table> {
        self.0.table_at(PROFILE_KEY)?.table_at(id)
    }

    #[cfg(any(windows, test))]
    fn ensure_profile_configs_table(&mut self, id: ProfileId) -> &mut toml::Table {
        self.0.ensure_table_at(PROFILE_KEY).ensure_table_at(id)
    }

    fn ensure_default(&mut self) {
        self.set_scripts_if_none(
            OperatingSystem::Windows,
            [PredefinedScript {
                label_template: "{reboot_action} no {next_boot_operating_system}".to_string(),
                script: Script {
                    next_boot_operating_system: Some(SetOrUnset::Set(OperatingSystem::Linux)),
                    reboot_action: Some(RebootAction::Reboot),
                    ..Script::new()
                },
            }],
        );

        self.set_scripts_if_none(OperatingSystem::Linux,
            ProfileId::values().map(|profile_id| {
                PredefinedScript {
                    script: Script {
                        next_boot_operating_system: Some(SetOrUnset::Set(OperatingSystem::Windows)),
                        next_windows_boot_profile: Some(SetOrUnset::Set(profile_id)),
                        switch_to_profile: None,
                        reboot_action: Some(RebootAction::Reboot),
                    },
                    label_template: "{reboot_action} no {next_boot_operating_system} usando o perfil {next_windows_boot_profile}".to_string(),
                }
            })
        );
    }

    fn set_scripts_if_none(
        &mut self,
        os: OperatingSystem,
        scripts: impl IntoIterator<Item = PredefinedScript>,
    ) {
        let has_scripts = self
            .operating_system_configs_table(os)
            .is_some_and(|os_table| os_table.get(SCRIPTS_KEY).is_some());

        if !has_scripts {
            let scripts = scripts.into_iter().collect::<Vec<_>>();
            self.ensure_operating_system_configs_table(os).insert(
                SCRIPTS_KEY.to_string(),
                toml::Value::try_from(scripts).unwrap(),
            );
        }
    }
}
impl From<toml::Table> for Content {
    fn from(value: toml::Table) -> Self {
        let mut content = Content(value);
        content.ensure_default();
        content
    }
}
impl Default for Content {
    fn default() -> Self {
        let mut content = Content(toml::Table::new());
        content.ensure_default();
        content
    }
}

trait TableExt {
    fn table_at<K: Serialize>(&self, key: K) -> Option<&toml::Table>;
    fn ensure_table_at<K: Serialize>(&mut self, key: K) -> &mut toml::Table;
}
impl TableExt for toml::Table {
    fn table_at<K: Serialize>(&self, key: K) -> Option<&toml::Table> {
        self.get(&key.serialize_to_string())?.as_table()
    }

    fn ensure_table_at<K: Serialize>(&mut self, key: K) -> &mut toml::Table {
        self.entry(key.serialize_to_string())
            .or_insert_with(|| toml::Table::new().into())
            .as_table_mut()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::script::SwitchToProfile;

    #[test]
    fn content_written_by_the_writer_can_be_read_by_the_reader() -> Result<()> {
        let expected = Configs {
            operating_system: OperatingSystemsConfigs {
                windows: OperatingSystemConfigs {
                    grub_entry: "windows-grub-entry".to_string(),
                    scripts: vec![PredefinedScript {
                        script: Script {
                            next_boot_operating_system: None,
                            next_windows_boot_profile: None,
                            switch_to_profile: None,
                            reboot_action: None,
                        },
                        label_template: "windows-script-label".to_string(),
                    }],
                },
                linux: OperatingSystemConfigs {
                    grub_entry: "linux-grub-entry".to_string(),
                    scripts: vec![PredefinedScript {
                        script: Script {
                            next_boot_operating_system: Some(SetOrUnset::Set(
                                OperatingSystem::Linux,
                            )),
                            next_windows_boot_profile: Some(SetOrUnset::Unset),
                            switch_to_profile: Some(SwitchToProfile::Other),
                            reboot_action: Some(RebootAction::Reboot),
                        },
                        label_template: "linux-script-label".to_string(),
                    }],
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
            },
        };

        // Sets the content via the writer.
        let writer = {
            let mut content = Content(toml::Table::new());
            for os in OperatingSystem::values() {
                content.set_scripts_if_none(os, expected.operating_system[os].scripts.clone());
            }

            let mut writer = ConfigsWriter { content };

            for os in OperatingSystem::values() {
                writer.set_grub_entry(os, &expected.operating_system[os].grub_entry);
            }
            for profile_id in ProfileId::values() {
                writer.set_profile_configs_strs(
                    profile_id,
                    &expected.profile[profile_id].label,
                    &expected.profile[profile_id].display_configs,
                )?;
            }

            writer
        };

        // Initializes the reader with the writer content.
        let configs = Configs::from_serialized(&writer.serialized()?)?;

        assert_eq!(configs, expected);
        Ok(())
    }

    #[test]
    fn writer_set_and_get_grub_entry() {
        let mut writer = ConfigsWriter {
            content: Content(toml::Table::new()),
        };
        assert!(!writer.has_grub_entry(OperatingSystem::Windows));
        assert!(!writer.has_grub_entry(OperatingSystem::Linux));

        writer.set_grub_entry(OperatingSystem::Windows, "windows-grub-entry");
        assert!(writer.has_grub_entry(OperatingSystem::Windows));
        assert!(!writer.has_grub_entry(OperatingSystem::Linux));

        writer.set_grub_entry(OperatingSystem::Linux, "linux-grub-entry");
        assert!(writer.has_grub_entry(OperatingSystem::Windows));
        assert!(writer.has_grub_entry(OperatingSystem::Linux));
    }

    #[test]
    fn writer_set_and_has_profile_configs() -> Result<()> {
        let mut writer = ConfigsWriter {
            content: Content(toml::Table::new()),
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

    mod content_set_scripts_if_none {
        use super::*;

        fn predef_script_with_label(label: &str) -> PredefinedScript {
            PredefinedScript {
                script: Script::new(),
                label_template: label.to_string(),
            }
        }

        macro_rules! get_scripts {
            ($content:expr, $os:expr) => {{
                let os = $content
                    .operating_system_configs_table($os)
                    .expect("should not be None");
                let scripts = os.get(SCRIPTS_KEY).expect("should not be None");
                scripts.as_array().expect("should be an array")
            }};
        }

        #[test]
        fn no_os_table() -> Result<()> {
            let mut content = Content(toml::Table::new());
            assert_eq!(
                content.operating_system_configs_table(OperatingSystem::Linux),
                None
            );
            let ps = predef_script_with_label("new");

            content.set_scripts_if_none(OperatingSystem::Linux, [ps.clone()]);

            let scripts = get_scripts!(content, OperatingSystem::Linux);
            assert_eq!(scripts.len(), 1);
            assert_eq!(scripts[0].clone().try_into::<PredefinedScript>()?, ps);
            Ok(())
        }

        #[test]
        fn os_table_exists() -> Result<()> {
            let mut content = Content(toml::Table::new());
            content.ensure_operating_system_configs_table(OperatingSystem::Linux);
            assert_ne!(
                content.operating_system_configs_table(OperatingSystem::Linux),
                None
            );
            let ps = predef_script_with_label("new");

            content.set_scripts_if_none(OperatingSystem::Linux, [ps.clone()]);

            let scripts = get_scripts!(content, OperatingSystem::Linux);
            assert_eq!(scripts.len(), 1);
            assert_eq!(scripts[0].clone().try_into::<PredefinedScript>()?, ps);
            Ok(())
        }

        #[test]
        fn scripts_exists() -> Result<()> {
            let mut content = Content(toml::Table::new());
            let ps = predef_script_with_label("new");
            content.set_scripts_if_none(OperatingSystem::Linux, [ps.clone()]);
            let scripts = get_scripts!(content, OperatingSystem::Linux);
            assert_eq!(scripts.len(), 1);
            assert_eq!(scripts[0].clone().try_into::<PredefinedScript>()?, ps);
            let ps_other = predef_script_with_label("other");

            content.set_scripts_if_none(OperatingSystem::Linux, [ps_other.clone()]);

            let scripts = get_scripts!(content, OperatingSystem::Linux);
            assert_eq!(scripts.len(), 1);
            assert_ne!(scripts[0].clone().try_into::<PredefinedScript>()?, ps_other);
            assert_eq!(scripts[0].clone().try_into::<PredefinedScript>()?, ps);
            Ok(())
        }
    }
}
