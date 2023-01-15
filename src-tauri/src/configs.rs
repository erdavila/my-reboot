use std::io;
use std::rc::Rc;

use crate::options_types::{OperatingSystem, OptionType};
use crate::properties::Properties;

const CONFIGS_FILENAME: &str = "my-reboot-configs.properties";

pub struct Configs {
    grub_entry_mapper: ConfigMapper<OperatingSystem>,
}

impl Configs {
    pub fn load(must_exist: bool) -> io::Result<Configs> {
        let props = Properties::load(CONFIGS_FILENAME, must_exist)?;
        Ok(Self::from_props(props))
    }

    fn from_props(props: Properties) -> Configs {
        let props = Rc::new(props);

        Configs {
            grub_entry_mapper: ConfigMapper::new(
                Rc::clone(&props),
                "grubEntry",
                OperatingSystem::values(),
                OperatingSystem::Linux,
            ),
        }
    }

    pub fn get_operating_system_by_grub_entry(&self, grub_entry: &str) -> OperatingSystem {
        self.grub_entry_mapper.get_object_by_value(grub_entry)
    }
}

// object.attribute
struct ConfigMapper<O: OptionType> {
    props: Rc<Properties>,
    attribute: &'static str,
    objects: Vec<O>,
    config_provider_os: OperatingSystem,
}
impl<O: OptionType + Copy> ConfigMapper<O> {
    fn new(
        props: Rc<Properties>,
        attribute: &'static str,
        objects: Vec<O>,
        config_provider_os: OperatingSystem,
    ) -> ConfigMapper<O> {
        ConfigMapper {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Copy, Clone, PartialEq, Eq, Debug)]
    enum OT {
        Value1,
        Value2,
    }
    impl OptionType for OT {
        fn values() -> Vec<Self> {
            todo!()
        }

        fn to_option_string(&self) -> &str {
            match self {
                OT::Value1 => "value1",
                OT::Value2 => "value2",
            }
        }
    }

    #[test]
    fn configs_get_operating_system_by_grub_entry() {
        let mut props = crate::properties::tests::create_empty_properties();
        for os in OperatingSystem::values() {
            props.set(
                &format!("{}.grubEntry", os.to_option_string()),
                &format!("{}-entry", os.to_string()),
            );
        }
        let configs = Configs::from_props(props);

        let os = configs.get_operating_system_by_grub_entry("Linux-entry");

        assert_eq!(os, OperatingSystem::Linux);
    }

    #[test]
    fn config_mapper_get_object_by_value() {
        let config_mapper = create_config_mapper();

        let object = config_mapper.get_object_by_value("the-value");

        assert_eq!(object, OT::Value1);
    }

    #[test]
    #[should_panic]
    fn config_mapper_get_object_by_value_not_found() {
        let config_mapper = create_config_mapper();

        config_mapper.get_object_by_value("some-value");
    }

    #[test]
    fn config_mapper_get_value() {
        let config_mapper = create_config_mapper();

        let value = config_mapper.get_value(OT::Value1);

        assert_eq!(value, "the-value");
    }

    #[test]
    #[should_panic]
    fn config_mapper_get_value_not_found() {
        let config_mapper = create_config_mapper();

        config_mapper.get_value(OT::Value2);
    }

    #[test]
    fn config_mapper_key_for() {
        let config_mapper = create_config_mapper();

        let key = config_mapper.key_for(OT::Value1);

        assert_eq!(key, "value1.attr");
    }

    fn create_config_mapper() -> ConfigMapper<OT> {
        let mut props = crate::properties::tests::create_empty_properties();
        props.set("value1.attr", "the-value");
        ConfigMapper {
            props: Rc::new(props),
            attribute: "attr",
            objects: vec![OT::Value1, OT::Value2],
            config_provider_os: OperatingSystem::Linux,
        }
    }
}
