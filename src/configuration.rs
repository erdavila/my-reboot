use anyhow::Result;
use rustyline::DefaultEditor;

use crate::host_os::{self, HOST_OS};
use crate::options_types::{OperatingSystem, OptionType, ProfileId};
use crate::persist::configs::ConfigsWriter;

pub(crate) fn configure() -> Result<()> {
    let mut configurer = Configurer::new()?;
    configurer.configure()
}

pub(crate) struct Configurer {
    pub(crate) configs: ConfigsWriter,
    pub(crate) readline: DefaultEditor,
}

impl Configurer {
    fn new() -> Result<Self> {
        let configs = ConfigsWriter::load()?;
        let readline = DefaultEditor::new()?;
        Ok(Configurer { configs, readline })
    }

    fn configure(&mut self) -> Result<()> {
        let result = self.do_configuration();
        self.show_configuration_status();
        result
    }

    fn do_configuration(&mut self) -> Result<()> {
        if self.is_configured(HOST_OS) {
            println!("A configuração no {HOST_OS} já está feita.");
            self.readline
                .readline("Tecle ENTER para refazê-la, ou Ctrl+C para cancelar.")?;
            println!();
        }

        host_os::configuration::configure(self)?;

        println!("Salvando configurações...");
        self.configs.save()?;
        Ok(())
    }

    fn show_configuration_status(&self) {
        println!();

        for os in OperatingSystem::values() {
            let status = if self.is_configured(os) {
                "✅ Feita"
            } else {
                "❌ Pendente"
            };

            println!("Configuração no {os}: {status}");
        }

        println!();
    }

    fn is_configured(&self, os: OperatingSystem) -> bool {
        match os {
            OperatingSystem::Windows => ProfileId::values()
                .into_iter()
                .all(|id| self.configs.has_profile_configs(id)),
            OperatingSystem::Linux => OperatingSystem::values()
                .into_iter()
                .all(|os| self.configs.has_grub_entry(os)),
        }
    }
}
