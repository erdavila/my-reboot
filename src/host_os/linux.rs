use std::process::Command;

use anyhow::Result;

use super::{PredefinedScript, SuccessOr};
use crate::options_types::{OperatingSystem, ProfileId, RebootAction};
use crate::script::{Script, SetOrUnset};
use crate::text;

pub const HOST_OS: OperatingSystem = OperatingSystem::Linux;
pub const STATE_DIR_PATH: &str = "/boot/grub/grubenv.dir";

pub const PREDEFINED_SCRIPTS: [PredefinedScript; 2] = [
    PredefinedScript {
        button_label_template: "{reboot_action} no {next_boot_operating_system} usando o perfil {next_windows_boot_profile}",
        script: reboot_on_windows_with_profile(ProfileId::A),
    },
    PredefinedScript {
        button_label_template: "{reboot_action} no {next_boot_operating_system} usando o perfil {next_windows_boot_profile}",
        script: reboot_on_windows_with_profile(ProfileId::B),
    },
];

const fn reboot_on_windows_with_profile(profile_id: ProfileId) -> Script {
    Script {
        next_boot_operating_system: Some(SetOrUnset::Set(OperatingSystem::Windows)),
        next_windows_boot_profile: Some(SetOrUnset::Set(profile_id)),
        reboot_action: Some(RebootAction::Reboot),
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

pub mod configuration {
    use std::collections::HashMap;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use anyhow::{Result, bail};
    use regex::Regex;

    use crate::configuration::Configurer;
    use crate::options_types::{OperatingSystem, OptionType};

    pub fn configure(configurer: &mut Configurer) -> Result<()> {
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

        for os in OperatingSystem::values() {
            if let Some(grub_entry) = entries.get(&os) {
                configurer.configs.set_grub_entry(os, grub_entry)?;
            } else {
                bail!("Entrada não encontrada para {os}");
            }
        }

        Ok(())
    }
}
