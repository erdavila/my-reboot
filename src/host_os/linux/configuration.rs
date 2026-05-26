use std::fs::File;
use std::io::{BufRead, BufReader};

use anyhow::{Result, anyhow};
use regex::Regex;

use crate::configuration::Configurer;
use crate::options_types::{OperatingSystem, Values as _};

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

    let mut entries = [None, None];
    for line in reader.lines() {
        let line = line?;
        if let Some((os, grub_entry)) = extract_os_and_grub_entry(&line) {
            entries[os as usize] = Some(grub_entry);
        }
    }

    for os in OperatingSystem::values() {
        let grub_entry = entries[os as usize]
            .as_ref()
            .ok_or_else(|| anyhow!("Entrada não encontrada para {os}"))?;
        configurer.configs.set_grub_entry(os, grub_entry);
    }

    Ok(())
}
