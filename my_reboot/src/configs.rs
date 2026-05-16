use std::borrow::Borrow;
use std::fmt::Debug;
use std::io;
use std::marker::PhantomData;

#[cfg(windows)]
use display_profile_lib::Profile;

#[cfg(windows)]
use crate::options_types::Display;
use crate::options_types::{OperatingSystem, OptionType, ProfileId};
use crate::properties::Properties;

const CONFIGS_FILENAME: &str = "my-reboot-configs.properties";

pub struct Configs {
    props: Properties,
    grub_entry_handler: ConfigHandler<OperatingSystem>,
    #[cfg(windows)]
    profile_configs_handler: ConfigHandler<ProfileId, ProfileSerialization>,
    profile_label_handler: ConfigHandler<ProfileId>,
    #[cfg(windows)]
    device_id_handler: ConfigHandler<Display>,
    #[cfg(windows)]
    display_switch_arg_handler: ConfigHandler<Display>,
}

impl Configs {
    pub fn load(must_exist: bool) -> io::Result<Configs> {
        let props = Properties::load(CONFIGS_FILENAME, must_exist)?;
        Ok(Configs {
            props,
            grub_entry_handler: ConfigHandler::new("grubEntry", OperatingSystem::Linux),
            #[cfg(windows)]
            profile_configs_handler: ConfigHandler::new("configs", OperatingSystem::Windows),
            profile_label_handler: ConfigHandler::new("label", OperatingSystem::Windows),
            #[cfg(windows)]
            device_id_handler: ConfigHandler::new("deviceId", OperatingSystem::Windows),
            #[cfg(windows)]
            display_switch_arg_handler: ConfigHandler::new(
                "displaySwitchArg",
                OperatingSystem::Windows,
            ),
        })
    }

    pub fn get_operating_system_by_grub_entry(&self, grub_entry: &str) -> OperatingSystem {
        self.grub_entry_handler
            .get_object_by_value(grub_entry, &self.props)
    }

    pub fn get_grub_entry(&self, os: OperatingSystem) -> &str {
        self.grub_entry_handler.get_value(os, &self.props)
    }

    #[cfg(windows)]
    pub(crate) fn get_profile_id(&self, profile: &Profile) -> Option<ProfileId> {
        self.profile_configs_handler
            .get_object_by_value_opt(profile, &self.props)
    }

    pub(crate) fn get_profile_label(&self, profile_id: ProfileId) -> &str {
        self.profile_label_handler
            .get_value(profile_id, &self.props)
    }

    pub(crate) fn get_profile_label_opt(&self, profile_id: ProfileId) -> Option<&str> {
        self.profile_label_handler
            .get_value_opt(profile_id, &self.props)
    }

    #[cfg(windows)]
    pub fn get_display_by_device_id(&self, device_id: &str) -> Display {
        self.device_id_handler
            .get_object_by_value(device_id, &self.props)
    }

    #[cfg(windows)]
    pub fn get_display_switch_arg(&self, display: Display) -> &str {
        self.display_switch_arg_handler
            .get_value(display, &self.props)
    }
}

pub struct ConfigsWriter {
    configs: Configs,
}
impl ConfigsWriter {
    pub fn load(must_exist: bool) -> io::Result<ConfigsWriter> {
        let configs = Configs::load(must_exist)?;
        Ok(Self { configs })
    }

    #[cfg(not(windows))]
    pub fn set_grub_entry(&mut self, os: OperatingSystem, value: &str) {
        self.configs
            .grub_entry_handler
            .set_value(os, value, &mut self.configs.props);
    }

    #[cfg(windows)]
    pub fn set_device_id(&mut self, display: Display, value: &str) {
        self.configs
            .device_id_handler
            .set_value(display, value, &mut self.configs.props);
    }

    #[cfg(windows)]
    pub fn set_display_switch_arg(&mut self, display: Display, value: &str) {
        self.configs
            .display_switch_arg_handler
            .set_value(display, value, &mut self.configs.props);
    }

    pub fn save(&mut self) -> io::Result<()> {
        self.configs.props.save()
    }
}

struct ConfigHandler<O, S = StringSerialization> {
    attribute: &'static str,
    config_provider_os: OperatingSystem,
    _option_type: PhantomData<(O, S)>,
}
impl<O, S> ConfigHandler<O, S>
where
    O: OptionType,
    S: Serialization,
{
    fn new(attribute: &'static str, config_provider_os: OperatingSystem) -> Self {
        Self {
            attribute,
            config_provider_os,
            _option_type: PhantomData,
        }
    }

    fn get_object_by_value<W>(&self, value: &W, props: &Properties) -> O
    where
        for<'a> S::DeserializeOutput<'a>: Borrow<W>,
        W: PartialEq + Debug + ?Sized,
    {
        self.get_object_by_value_opt(value, props)
            .unwrap_or_else(|| {
                panic!(
                    "{}",
                    configuration_error(
                        &format!(
                            "Configuração '{}' com valor {value:?} não encontrada",
                            self.attribute
                        ),
                        self.config_provider_os,
                    )
                )
            })
    }

    fn get_object_by_value_opt<W>(&self, value: &W, props: &Properties) -> Option<O>
    where
        for<'a> S::DeserializeOutput<'a>: Borrow<W>,
        W: PartialEq + Debug + ?Sized,
    {
        O::values()
            .into_iter()
            .find(|&o| self.get_value(o, props).borrow() == value)
    }

    fn get_value<'a>(&self, object: O, props: &'a Properties) -> S::DeserializeOutput<'a> {
        self.get_value_opt(object, props).unwrap_or_else(|| {
            let key = self.key_for(object);
            panic!(
                "{}",
                configuration_error(
                    &format!("Configuração '{key}' não encontrada"),
                    self.config_provider_os,
                )
            )
        })
    }

    fn get_value_opt<'a>(
        &self,
        object: O,
        props: &'a Properties,
    ) -> Option<S::DeserializeOutput<'a>> {
        let key = self.key_for(object);
        props.get(&key).map(|str| S::deserialize(str))
    }

    fn set_value(&mut self, object: O, value: S::SerializeInput<'_>, props: &mut Properties) {
        let key = self.key_for(object);
        let value = S::serialize(value);
        props.set(&key, value.as_ref());
    }

    fn key_for(&self, object: O) -> String {
        format!("{}.{}", object.to_option_string(), self.attribute)
    }
}

trait Serialization {
    type SerializeInput<'a>;
    type SerializeOutput<'a>: AsRef<str>;
    type DeserializeOutput<'a>;

    fn serialize(value: Self::SerializeInput<'_>) -> Self::SerializeOutput<'_>;
    fn deserialize(str: &str) -> Self::DeserializeOutput<'_>;
}

struct StringSerialization;
impl Serialization for StringSerialization {
    type SerializeInput<'a> = &'a str;
    type SerializeOutput<'a> = &'a str;
    type DeserializeOutput<'a> = &'a str;

    fn serialize(value: Self::SerializeInput<'_>) -> Self::SerializeOutput<'_> {
        value
    }

    fn deserialize(str: &str) -> Self::DeserializeOutput<'_> {
        str
    }
}

#[cfg(windows)]
struct ProfileSerialization;
#[cfg(windows)]
impl Serialization for ProfileSerialization {
    type SerializeInput<'a> = &'a Profile;
    type SerializeOutput<'a> = String;
    type DeserializeOutput<'a> = Profile;

    fn serialize(value: Self::SerializeInput<'_>) -> Self::SerializeOutput<'_> {
        serde_json::to_string(value).unwrap()
    }

    fn deserialize(str: &str) -> Self::DeserializeOutput<'_> {
        serde_json::from_str(str).unwrap()
    }
}

fn configuration_error(message: &str, config_provider_os: OperatingSystem) -> String {
    format!("{message}. Execute 'my-reboot configure' no {config_provider_os}")
}
