mod args;
mod configuration;
mod dialog;
mod host_os;
mod options_types;
mod script;
mod state;
mod text;

mod persist {
    pub(crate) mod configs;
    pub(crate) mod grubenv;
    pub(crate) mod options;
}

use std::fmt::Display;
use std::io;
use std::num::NonZeroUsize;

use ansi_term::Color::Red;
use anyhow::{Context, Result, bail};
use dialog::Mode;
use script::Script;
#[cfg(all(windows, not(test)))]
use script::SwitchToProfile;

use crate::args::{ParsedArgs, PredefinedScriptParsedArgs, ShowArgs};
use crate::host_os::HOST_OS;
use crate::options_types::{OperatingSystem, ProfileId, Values as _};
use crate::persist::configs::{Configs, UntypedConfigs, is_not_found_io_error};
use crate::state::StateProvider;
use crate::text::{Capitalized, IndentedBlockWriter, IndentedBlockWriterExt as _, Quoted};

fn main() -> Result<()> {
    let args = args::parse()
        .with_context(|| "Argumentos inválidos.\nPara ajuda, execute: my-reboot --help")?;

    match args {
        ParsedArgs::Dialog(mode) => show_dialog(mode),
        ParsedArgs::Script(script) => execute_script(script),
        ParsedArgs::PredefinedScript(PredefinedScriptParsedArgs::Number(number)) => {
            execute_predefined_script(number)
        }
        ParsedArgs::PredefinedScript(PredefinedScriptParsedArgs::List) => list_predefined_scripts(),
        ParsedArgs::Show(ShowArgs::Options) => show_options(),
        ParsedArgs::Show(ShowArgs::Configs) => show_configs(),
        ParsedArgs::Configure => configure(),
        ParsedArgs::Usage => {
            show_usage();
            Ok(())
        }
        ParsedArgs::Version => {
            show_version();
            Ok(())
        }
    }
}

fn show_dialog(mode: Mode) -> Result<()> {
    #[cfg(windows)]
    {
        // Hide the console window.
        use windows::Win32::System::Console::GetConsoleWindow;
        use windows::Win32::UI::WindowsAndMessaging::{SW_HIDE, ShowWindow};
        unsafe {
            let window = GetConsoleWindow();
            if !window.is_invalid() {
                let _ = ShowWindow(window, SW_HIDE);
            }
        }
    }

    let provider = StateProvider::new()?;

    let labels: Vec<_> = provider.configs().operating_system[HOST_OS]
        .scripts
        .iter()
        .map(|ps| ps.resolve_label(provider.configs()))
        .collect();

    let state = provider.state()?;
    let script_options = dialog::ScriptOptions {
        next_boot_operating_system: state.next_boot_operating_system,
        next_windows_boot_profile: state.next_windows_boot_profile,
        #[cfg(windows)]
        switch_profile: false,
        reboot_action: None,
    };
    let profile_labels = ProfileId::values()
        .map(|id| text::profile::labeled_profile(id, id.label(provider.configs())));

    let outcome = dialog::show(mode, labels, script_options, profile_labels)?;

    match outcome {
        Some(dialog::Outcome::PredefinedScriptIndex(index)) => {
            provider.configs().operating_system[HOST_OS].scripts[index]
                .script
                .execute()
        }
        Some(dialog::Outcome::ScriptOptions(options)) => {
            let script = Script {
                next_boot_operating_system: Some(options.next_boot_operating_system.into()),
                next_windows_boot_profile: Some(options.next_windows_boot_profile.into()),
                switch_to_profile: cfg_select! {
                    all(windows, not(test)) => options.switch_profile.then_some(SwitchToProfile::Other),
                    _ => None,
                },
                reboot_action: options.reboot_action,
            };
            script.execute()
        }
        None => Ok(()),
    }
}

fn execute_predefined_script(number: NonZeroUsize) -> Result<()> {
    let index = number.get() - 1;

    let configs = Configs::load()?;
    let predef_scripts = &configs.operating_system[HOST_OS].scripts;
    let Some(predef_script) = predef_scripts.get(index) else {
        bail!(
            "Número inválido de script para o sistema operacional atual (mín: 1; máx: {})",
            predef_scripts.len()
        );
    };

    println!(
        "Executando script '{}'",
        predef_script.resolve_label(&configs)
    );
    predef_script.script.execute()
}

fn list_predefined_scripts() -> Result<()> {
    let configs = Configs::load()?;
    let mut writer = IndentedBlockWriter::from(std::io::stdout());

    writer.write_predefined_scripts(&configs.operating_system[HOST_OS].scripts, &configs)?;

    Ok(())
}

fn execute_script(script: Script) -> Result<()> {
    script.execute()
}

fn show_options() -> Result<()> {
    let provider = StateProvider::new()?;
    let state = provider.state()?;

    println!(
        "{}: {}",
        Capitalized(text::operating_system::ON_NEXT_BOOT_DESCRIPTION),
        text::operating_system::value_text(state.next_boot_operating_system)
    );
    println!(
        "{}: {}",
        Capitalized(text::profile::ON_NEXT_WINDOWS_BOOT_DESCRIPTION),
        text::profile::next_boot_value_text(
            state
                .next_windows_boot_profile
                .map(|id| (id, id.label(provider.configs())))
        )
    );
    #[cfg(windows)]
    println!(
        "{}: {}",
        Capitalized(text::profile::CURRENT),
        text::profile::current_value_text(
            state
                .current_profile
                .map(|id| (id, id.label(provider.configs())))
        )
    );

    Ok(())
}

fn show_configs() -> Result<()> {
    print!("Arquivo de configurações: {}", Configs::path().display());
    let configs = match UntypedConfigs::load() {
        Ok(configs) => configs,
        Err(e) => {
            if is_not_found_io_error(&e) {
                println!(" {}", Red.paint("(inexistente)"));
                return Ok(());
            }
            return Err(e);
        }
    };
    println!();
    println!();

    let missing_config = Red.paint("configuração faltando");
    let mut writer = IndentedBlockWriter::from(std::io::stdout());

    writer.write_block("Entradas do GRUB", |w| {
        for os in OperatingSystem::values() {
            let grub_entry = configs.grub_entry(os);
            let entry: &dyn Display = grub_entry.as_ref().map_or(&missing_config, |entry| entry);
            w.write(format_args!("{os}: {entry}"))?;
        }
        w.write("")
    })?;

    writer.write_block("Perfis", |w| {
        for id in ProfileId::values() {
            cfg_select! {
                windows => {
                    if let Some(profile_configs) = configs.profile_configs(id) {
                        let (label, profile) = profile_configs.map_err(io::Error::other)?;
                        w.write_profile_summary(format_args!("{id}: {}", Quoted(label)), &profile)?;
                    } else {
                        w.write(format_args!("{id}: {missing_config}"))?;
                    }
                },
                _ => {
                    if let Some(profile_configs) = configs.profile_configs_strs(id) {
                        let profile_configs = profile_configs.map_err(io::Error::other)?;
                        let label = &profile_configs.label;
                        w.write(format_args!("{id}: {}", Quoted(label)))?;
                    } else {
                        w.write(format_args!("{id}: {missing_config}"))?;
                    }
                }
            }
        }
        w.write("")
    })?;

    writer.write_block("Scripts pré-definidos", |w| {
        for os in OperatingSystem::values() {
            if let Some(scripts) = configs.scripts(os) {
                let scripts = scripts.map_err(io::Error::other)?;
                if scripts.is_empty() {
                    w.write(format_args!("{os}: nenhum"))?;
                } else {
                    w.write_block(os, |w| w.write_predefined_scripts(&scripts, &configs))?;
                }
            } else {
                w.write(format_args!("{os}: {}", Red.paint("indefinidos")))?;
            }
        }
        Ok(())
    })?;

    Ok(())
}

fn configure() -> Result<()> {
    configuration::configure()
}

fn show_usage() {
    let profile_labels =
        Configs::load().map(|configs| [configs.profile.a.label, configs.profile.b.label]);

    let usage = args::Usage::new(profile_labels);

    println!("{usage}");
}

fn show_version() {
    println!(
        "{} {} ({})",
        env!("MY_REBOOT_NAME"),
        env!("MY_REBOOT_VERSION"),
        env!("MY_REBOOT_TIMESTAMP")
    );

    let vcs_revision = env!("MY_REBOOT_VCS_REVISION");
    if let Some(jj_ids) = vcs_revision.strip_prefix("JJ:") {
        let (change_id, commit_id) = jj_ids.split_once(' ').unwrap();
        println!("JJ change/commit ID: {change_id}/{commit_id}");
    } else {
        let git_head = vcs_revision.strip_prefix("Git:").unwrap();
        println!("Git HEAD: {git_head}");
    }
}
