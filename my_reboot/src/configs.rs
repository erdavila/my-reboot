use std::io;
use std::marker::PhantomData;

#[cfg(windows)]
use crate::options_types::Display;
use crate::options_types::{OperatingSystem, OptionType};
use crate::properties::Properties;

const CONFIGS_FILENAME: &str = "my-reboot-configs.properties";

pub struct Configs {
    props: Properties,
    grub_entry_handler: ConfigHandler<OperatingSystem>,
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

    pub fn get_grub_entry(&self, os: OperatingSystem) -> String {
        self.grub_entry_handler.get_value(os, &self.props)
    }

    #[cfg(windows)]
    pub fn get_display_by_device_id(&self, device_id: &str) -> Display {
        self.device_id_handler
            .get_object_by_value(device_id, &self.props)
    }

    #[cfg(windows)]
    pub fn get_display_switch_arg(&self, display: Display) -> String {
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

struct ConfigHandler<O: OptionType> {
    attribute: &'static str,
    config_provider_os: OperatingSystem,
    _option_type: PhantomData<O>,
}
impl<O: OptionType> ConfigHandler<O> {
    fn new(attribute: &'static str, config_provider_os: OperatingSystem) -> Self {
        Self {
            attribute,
            config_provider_os,
            _option_type: PhantomData,
        }
    }

    fn get_object_by_value(&self, value: &str, props: &Properties) -> O {
        O::values()
            .into_iter()
            .find(|o| self.get_value(*o, props) == value)
            .unwrap_or_else(|| {
                panic!(
                    "{}",
                    configuration_error(
                        &format!("Configuração com valor {value} não encontrada"),
                        self.config_provider_os,
                    )
                )
            })
    }

    fn get_value(&self, object: O, props: &Properties) -> String {
        let key = self.key_for(object);
        props
            .get(&key)
            .unwrap_or_else(|| {
                panic!(
                    "{}",
                    configuration_error(
                        &format!("Configuração '{key}' não encontrada"),
                        self.config_provider_os,
                    )
                )
            })
            .to_string()
    }

    fn set_value(&mut self, object: O, value: &str, props: &mut Properties) {
        let key = self.key_for(object);
        props.set(&key, value);
    }

    fn key_for(&self, object: O) -> String {
        format!("{}.{}", object.to_option_string(), self.attribute)
    }
}

fn configuration_error(message: &str, config_provider_os: OperatingSystem) -> String {
    format!(
        "{message}. Execute 'my-reboot configure' no {}",
        config_provider_os
    )
}
