use std::process::Command;

use anyhow::Result;

use crate::options_types::{Display, OperatingSystem, RebootAction};
use crate::script::{Script, SetOrUnset};
use crate::{configs::Configs, text};

use super::{CurrentDisplayHandler, PredefinedScript, SuccessOr};

pub const STATE_DIR_PATH: &str = "/boot/grub/grubenv.dir";

pub const PREDEFINED_SCRIPTS: [PredefinedScript; 2] = [
    PredefinedScript {
        button_label: "Reiniciar no Windows usando o monitor",
        script: reboot_on_windows_with_display(Display::Monitor),
    },
    PredefinedScript {
        button_label: "Reiniciar no Windows usando a TV",
        script: reboot_on_windows_with_display(Display::TV),
    },
];

const fn reboot_on_windows_with_display(display: Display) -> Script {
    Script {
        next_boot_operating_system: Some(SetOrUnset::Set(OperatingSystem::Windows)),
        next_windows_boot_display: Some(SetOrUnset::Set(display)),
        reboot_action: Some(RebootAction::Reboot),
        ..Script::new()
    }
}

pub fn reboot() -> Result<()> {
    systemctl("reboot")
}

pub fn shutdown() -> Result<()> {
    systemctl("poweroff")
}

fn systemctl(arg: &str) -> Result<()> {
    Command::new("systemctl")
        .arg(arg)
        .status()?
        .success_or(text::reboot_action::FAILED)
}

pub fn get_current_display_handler<'a>(
    _: &'a Configs,
) -> Option<Box<dyn CurrentDisplayHandler + 'a>> {
    None
}

pub mod configuration {
    use std::collections::HashMap;
    use std::error::Error;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use anyhow::Result;
    use regex::Regex;

    use crate::configs::ConfigsWriter;
    use crate::options_types::{OperatingSystem, OptionType};

    #[derive(Debug)]
    struct GrubEntryNotFound(OperatingSystem);
    impl std::fmt::Display for GrubEntryNotFound {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "Entrada não encontrada para {}", self.0)
        }
    }
    impl Error for GrubEntryNotFound {}

    pub fn configure() -> Result<()> {
        const GRUB_CFG: &str = "/boot/grub/grub.cfg";

        let grub_entry_re = Regex::new(r".*'([a-zA-Z0-9_-]+)'\s*\{.*")?;
        let extract_os_and_grub_entry = |line: &str| -> Option<(OperatingSystem, String)> {
            if !line.starts_with("menuentry ") {
                return None;
            }

            let uppercase_line = line.to_uppercase();
            let os = OperatingSystem::values()
                .into_iter()
                .find(|os| uppercase_line.contains(&os.to_string().to_uppercase()));

            let grub_entry = grub_entry_re.captures(line).map(|caps| caps[1].to_string());

            os.zip(grub_entry)
        };

        println!("Lendo {GRUB_CFG}...");
        let reader = BufReader::new(File::open(GRUB_CFG)?);
        let mut entries = HashMap::new();
        for line in reader.lines() {
            let line = line?;
            if let Some((os, grub_entry)) = extract_os_and_grub_entry(&line) {
                entries.insert(os, grub_entry);
            }
        }

        let mut configs = ConfigsWriter::load(false)?;
        for os in OperatingSystem::values() {
            if let Some(grub_entry) = entries.get(&os) {
                configs.set_grub_entry(os, grub_entry);
            } else {
                return Err(GrubEntryNotFound(os).into());
            }
        }

        for (os, grub_entry) in entries {
            configs.set_grub_entry(os, &grub_entry);
        }
        println!("Salvando configurações...");
        configs.save()?;

        println!("Configuração finalizada.");
        Ok(())
    }
}
