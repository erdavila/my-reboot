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

pub(crate) struct UntypedConfigs(toml::Table);
impl UntypedConfigs {
    pub(crate) fn load_or_default() -> Result<UntypedConfigs> {
        let content = Self::load_content().or_else(|e| {
            if is_not_found_io_error(&e) {
                Ok(toml::Table::new())
            } else {
                Err(e)
            }
        })?;

        let mut configs = UntypedConfigs(content);
        configs.ensure_defaults();

        Ok(configs)
    }

    #[cfg(test)]
    fn empty() -> UntypedConfigs {
        UntypedConfigs(toml::Table::new())
    }

    fn load_content() -> Result<toml::Table> {
        let string_content = fs::read_to_string(Configs::path())?;
        let content = toml::from_str(&string_content)?;
        Ok(content)
    }

    #[cfg(any(not(windows), test))]
    pub(crate) fn set_grub_entry(&mut self, os: OperatingSystem, grub_entry: &str) {
        self.ensure_operating_system_configs_table(os)
            .insert(GRUB_ENTRY_KEY.to_string(), grub_entry.into());
    }

    pub(crate) fn grub_entry(&self, os: OperatingSystem) -> Option<&str> {
        let os_configs = self.operating_system_configs_table(os)?;
        let value = os_configs.get(GRUB_ENTRY_KEY)?;
        value.as_str()
    }

    pub(crate) fn has_grub_entry(&self, os: OperatingSystem) -> bool {
        self.grub_entry(os).is_some()
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
        let profile_table = self.ensure_profile_configs_table(id);
        *profile_table = toml::Table::try_from(ProfileConfigs {
            label: label.to_string(),
            display_configs: configs.to_string(),
        })?;

        Ok(())
    }

    #[cfg(windows)]
    pub(crate) fn profile_configs(&self, id: ProfileId) -> Option<Result<(String, Profile)>> {
        self.profile_configs_strs(id).map(|result| {
            result.and_then(|cfgs| {
                cfgs.display_configs()
                    .map(|display_configs| (cfgs.label, display_configs))
            })
        })
    }

    fn profile_configs_strs(&self, id: ProfileId) -> Option<Result<ProfileConfigs>> {
        self.profile_configs_table(id).map(|profile_configs| {
            profile_configs
                .clone()
                .try_into::<ProfileConfigs>()
                .map_err(Into::into)
        })
    }

    pub(crate) fn has_profile_configs(&self, id: ProfileId) -> bool {
        self.profile_configs_strs(id)
            .is_some_and(|result| result.is_ok())
    }

    fn set_scripts(
        &mut self,
        os: OperatingSystem,
        scripts: impl IntoIterator<Item = PredefinedScript>,
    ) -> Result<()> {
        let scripts: Vec<_> = scripts.into_iter().collect();
        self.ensure_operating_system_configs_table(os)
            .insert(SCRIPTS_KEY.to_string(), toml::Value::try_from(scripts)?);
        Ok(())
    }

    fn set_scripts_if_none(
        &mut self,
        os: OperatingSystem,
        scripts: impl IntoIterator<Item = PredefinedScript>,
    ) {
        if self.scripts(os).is_none() {
            self.set_scripts(os, scripts).unwrap();
        }
    }

    pub(crate) fn scripts(&self, os: OperatingSystem) -> Option<Result<Vec<PredefinedScript>>> {
        let os_configs = self.operating_system_configs_table(os)?;
        let value = os_configs.get(SCRIPTS_KEY)?;
        let script_values = value.as_array()?;
        let scripts = script_values
            .iter()
            .map(|value| {
                value
                    .clone()
                    .try_into::<PredefinedScript>()
                    .map_err(Into::into)
            })
            .collect();
        Some(scripts)
    }

    fn ensure_defaults(&mut self) {
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

    pub(crate) fn save(&self) -> Result<()> {
        fs::write(Configs::path(), self.serialized()?)?;
        Ok(())
    }

    fn serialized(&self) -> Result<String> {
        let content = toml::to_string(&self.0)?;
        Ok(content)
    }
}

fn is_not_found_io_error(e: &anyhow::Error) -> bool {
    if let Some(io_error) = e.downcast_ref::<io::Error>() {
        io_error.kind() == io::ErrorKind::NotFound
    } else {
        false
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

    fn predef_script_with_label(label: &str) -> PredefinedScript {
        PredefinedScript {
            script: Script::new(),
            label_template: label.to_string(),
        }
    }

    #[test]
    fn content_set_by_the_untyped_configs_can_be_read_by_the_typed_configs() -> Result<()> {
        let expected = Configs {
            operating_system: OperatingSystemsConfigs {
                windows: OperatingSystemConfigs {
                    grub_entry: "windows-grub-entry".to_string(),
                    scripts: vec![predef_script_with_label("windows-script-label")],
                },
                linux: OperatingSystemConfigs {
                    grub_entry: "linux-grub-entry".to_string(),
                    scripts: vec![predef_script_with_label("linux-script-label")],
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

        // Sets the content via the untyped_configs.
        let untyped_configs = {
            let mut untyped_configs = UntypedConfigs::empty();

            for os in OperatingSystem::values() {
                untyped_configs.set_scripts(os, expected.operating_system[os].scripts.clone())?;
                untyped_configs.set_grub_entry(os, &expected.operating_system[os].grub_entry);
            }
            for profile_id in ProfileId::values() {
                untyped_configs.set_profile_configs_strs(
                    profile_id,
                    &expected.profile[profile_id].label,
                    &expected.profile[profile_id].display_configs,
                )?;
            }

            untyped_configs
        };

        // Initializes the typed configs with the untyped configs content.
        let configs = Configs::from_serialized(&untyped_configs.serialized()?)?;

        assert_eq!(configs, expected);
        Ok(())
    }

    #[test]
    fn untyped_configs_grub_entry() {
        let mut configs = UntypedConfigs::empty();
        assert!(!configs.has_grub_entry(OperatingSystem::Windows));
        assert!(!configs.has_grub_entry(OperatingSystem::Linux));

        configs.set_grub_entry(OperatingSystem::Windows, "windows-grub-entry");
        assert!(configs.has_grub_entry(OperatingSystem::Windows));
        assert!(!configs.has_grub_entry(OperatingSystem::Linux));

        configs.set_grub_entry(OperatingSystem::Linux, "linux-grub-entry");
        assert!(configs.has_grub_entry(OperatingSystem::Windows));
        assert!(configs.has_grub_entry(OperatingSystem::Linux));
    }

    #[test]
    #[allow(clippy::similar_names)]
    fn untyped_configs_profile_configs() -> Result<()> {
        let mut configs = UntypedConfigs::empty();
        assert!(configs.profile_configs_strs(ProfileId::A).is_none());
        assert!(configs.profile_configs_strs(ProfileId::B).is_none());
        assert!(!configs.has_profile_configs(ProfileId::A));
        assert!(!configs.has_profile_configs(ProfileId::B));

        let profile_a_configs = ProfileConfigs {
            label: "profile-a-label".to_string(),
            display_configs: "profile-a-display-configs".to_string(),
        };
        configs.set_profile_configs_strs(
            ProfileId::A,
            &profile_a_configs.label,
            &profile_a_configs.display_configs,
        )?;
        assert!(
            configs
                .profile_configs_strs(ProfileId::A)
                .is_some_and(|result| result.is_ok_and(|cfgs| cfgs == profile_a_configs))
        );
        assert!(configs.profile_configs_strs(ProfileId::B).is_none());
        assert!(configs.has_profile_configs(ProfileId::A));
        assert!(!configs.has_profile_configs(ProfileId::B));

        let profile_b_configs = ProfileConfigs {
            label: "profile-b-label".to_string(),
            display_configs: "profile-b-display-configs".to_string(),
        };
        configs.set_profile_configs_strs(
            ProfileId::B,
            &profile_b_configs.label,
            &profile_b_configs.display_configs,
        )?;
        assert!(
            configs
                .profile_configs_strs(ProfileId::A)
                .is_some_and(|result| result.is_ok_and(|cfgs| cfgs == profile_a_configs))
        );
        assert!(
            configs
                .profile_configs_strs(ProfileId::B)
                .is_some_and(|result| result.is_ok_and(|cfgs| cfgs == profile_b_configs))
        );
        assert!(configs.has_profile_configs(ProfileId::A));
        assert!(configs.has_profile_configs(ProfileId::B));

        Ok(())
    }

    #[test]
    fn untyped_configs_scripts() -> Result<()> {
        let mut configs = UntypedConfigs::empty();

        assert!(configs.scripts(OperatingSystem::Windows).is_none());
        assert!(configs.scripts(OperatingSystem::Linux).is_none());

        let windows_scripts = vec![predef_script_with_label("windows script")];
        configs.set_scripts(OperatingSystem::Windows, windows_scripts.clone())?;
        assert!(
            configs
                .scripts(OperatingSystem::Windows)
                .is_some_and(|result| result.is_ok_and(|scripts| scripts == windows_scripts))
        );
        assert!(configs.scripts(OperatingSystem::Linux).is_none());

        let linux_scripts = vec![predef_script_with_label("linux script")];
        configs.set_scripts(OperatingSystem::Linux, linux_scripts.clone())?;
        assert!(
            configs
                .scripts(OperatingSystem::Windows)
                .is_some_and(|result| result.is_ok_and(|scripts| scripts == windows_scripts))
        );
        assert!(
            configs
                .scripts(OperatingSystem::Linux)
                .is_some_and(|result| result.is_ok_and(|scripts| scripts == linux_scripts))
        );
        Ok(())
    }

    mod untyped_configs_set_scripts_if_none {
        use super::*;

        macro_rules! get_scripts {
            ($configs:expr, $os:expr) => {{
                let os = $configs
                    .operating_system_configs_table($os)
                    .expect("should not be None");
                let scripts = os.get(SCRIPTS_KEY).expect("should not be None");
                scripts.as_array().expect("should be an array")
            }};
        }

        #[test]
        fn no_os_table() -> Result<()> {
            let mut configs = UntypedConfigs::empty();
            assert_eq!(
                configs.operating_system_configs_table(OperatingSystem::Linux),
                None
            );
            let ps = predef_script_with_label("new");

            configs.set_scripts_if_none(OperatingSystem::Linux, [ps.clone()]);

            let scripts = get_scripts!(configs, OperatingSystem::Linux);
            assert_eq!(scripts.len(), 1);
            assert_eq!(scripts[0].clone().try_into::<PredefinedScript>()?, ps);
            Ok(())
        }

        #[test]
        fn os_table_exists() -> Result<()> {
            let mut configs = UntypedConfigs::empty();
            configs.ensure_operating_system_configs_table(OperatingSystem::Linux);
            assert_ne!(
                configs.operating_system_configs_table(OperatingSystem::Linux),
                None
            );
            let ps = predef_script_with_label("new");

            configs.set_scripts_if_none(OperatingSystem::Linux, [ps.clone()]);

            let scripts = get_scripts!(configs, OperatingSystem::Linux);
            assert_eq!(scripts.len(), 1);
            assert_eq!(scripts[0].clone().try_into::<PredefinedScript>()?, ps);
            Ok(())
        }

        #[test]
        fn scripts_exists() -> Result<()> {
            let mut configs = UntypedConfigs::empty();
            let ps = predef_script_with_label("new");
            configs.set_scripts_if_none(OperatingSystem::Linux, [ps.clone()]);
            let scripts = get_scripts!(configs, OperatingSystem::Linux);
            assert_eq!(scripts.len(), 1);
            assert_eq!(scripts[0].clone().try_into::<PredefinedScript>()?, ps);
            let ps_other = predef_script_with_label("other");

            configs.set_scripts_if_none(OperatingSystem::Linux, [ps_other.clone()]);

            let scripts = get_scripts!(configs, OperatingSystem::Linux);
            assert_eq!(scripts.len(), 1);
            assert_ne!(scripts[0].clone().try_into::<PredefinedScript>()?, ps_other);
            assert_eq!(scripts[0].clone().try_into::<PredefinedScript>()?, ps);
            Ok(())
        }
    }
}
