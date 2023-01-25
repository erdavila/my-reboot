use std::io;
use std::rc::Rc;

#[cfg(windows)]
use crate::options_types::Display;
use crate::options_types::{OperatingSystem, OptionType};
use crate::properties::Properties;

const CONFIGS_FILENAME: &str = "my-reboot-configs.properties";

pub struct Configs {
    grub_entry_handler: ConfigHandler<OperatingSystem>,
    #[cfg(windows)]
    device_id_handler: ConfigHandler<Display>,
    #[cfg(windows)]
    display_switch_arg_handler: ConfigHandler<Display>,
}

impl Configs {
    pub fn load(must_exist: bool) -> io::Result<Configs> {
        let props = Properties::load(CONFIGS_FILENAME, must_exist)?;
        let props = Rc::new(props);

        Ok(Configs {
            grub_entry_handler: ConfigHandler::new(
                Rc::clone(&props),
                "grubEntry",
                OperatingSystem::values(),
                OperatingSystem::Linux,
            ),
            #[cfg(windows)]
            device_id_handler: ConfigHandler::new(
                Rc::clone(&props),
                "deviceId",
                Display::values(),
                OperatingSystem::Windows,
            ),
            #[cfg(windows)]
            display_switch_arg_handler: ConfigHandler::new(
                Rc::clone(&props),
                "displaySwitchArg",
                Display::values(),
                OperatingSystem::Windows,
            ),
        })
    }

    pub fn get_operating_system_by_grub_entry(&self, grub_entry: &str) -> OperatingSystem {
        self.grub_entry_handler.get_object_by_value(grub_entry)
    }

    pub fn get_grub_entry(&self, os: OperatingSystem) -> &str {
        self.grub_entry_handler.get_value(os)
    }

    #[cfg(windows)]
    pub fn get_display_by_device_id(&self, device_id: &str) -> Display {
        self.device_id_handler.get_object_by_value(device_id)
    }

    #[cfg(windows)]
    pub fn get_display_switch_arg(&self, display: Display) -> &str {
        self.display_switch_arg_handler.get_value(display)
    }
}

struct ConfigHandler<O: OptionType> {
    props: Rc<Properties>,
    attribute: &'static str,
    objects: Vec<O>,
    config_provider_os: OperatingSystem,
}
impl<O: OptionType + Copy> ConfigHandler<O> {
    fn new(
        props: Rc<Properties>,
        attribute: &'static str,
        objects: Vec<O>,
        config_provider_os: OperatingSystem,
    ) -> ConfigHandler<O> {
        ConfigHandler {
            props,
            attribute,
            objects,
            config_provider_os,
        }
    }

    fn get_object_by_value(&self, value: &str) -> O {
        let object = self
            .objects
            .iter()
            .find(|o| self.get_value(**o) == value)
            .unwrap_or_else(|| {
                panic!(
                    "{}",
                    configuration_error(
                        &format!("Configuração com valor {value} não encontrada"),
                        self.config_provider_os,
                    )
                )
            });
        *object
    }

    fn get_value(&self, object: O) -> &str {
        let key = self.key_for(object);
        self.props.get(&key).unwrap_or_else(|| {
            panic!(
                "{}",
                configuration_error(
                    &format!("Configuração '{key}' não encontrada"),
                    self.config_provider_os,
                )
            )
        })
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
