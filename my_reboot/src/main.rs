#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod args;
mod configs;
mod dialog;
mod file_content_as_hash_map;
mod grubenv;
mod host_os;
mod options_types;
mod properties;
mod script;
mod state;
mod text;

use anyhow::{Context, Result};
use dialog::Mode;
use host_os::PREDEFINED_SCRIPTS;
use script::Script;
#[cfg(windows)]
use script::SwitchToDisplay;

use crate::args::ParsedArgs;
use crate::configs::Configs;
use crate::options_types::{LabeledProfile, ProfileId};
use crate::state::StateProvider;

fn main() -> Result<()> {
    let args = args::parse()
        .with_context(|| "Argumentos inválidos.\nPara ajuda, execute: my-reboot --help")?;

    match args {
        ParsedArgs::Dialog(mode) => show_dialog(mode),
        ParsedArgs::Script(script) => execute_script(script),
        ParsedArgs::ShowState => show_state(),
        #[cfg(not(windows))]
        ParsedArgs::Configure => configure(),
        #[cfg(windows)]
        ParsedArgs::Configure { initial_display } => configure(initial_display),
        ParsedArgs::Usage => show_usage(),
    }
}

fn show_dialog(mode: Mode) -> Result<()> {
    let labels: Vec<_> = PREDEFINED_SCRIPTS
        .iter()
        .map(|ps| ps.button_label)
        .collect();

    let provider = StateProvider::new()?;
    let state = provider.get_state()?;
    let script_options = dialog::ScriptOptions {
        next_boot_operating_system: state.next_boot_operating_system,
        next_windows_boot_display: state.next_windows_boot_display,
        #[cfg(windows)]
        switch_display: false,
        reboot_action: None,
    };

    let outcome = dialog::show(mode, labels, script_options)?;

    match outcome {
        Some(dialog::Outcome::PredefinedScriptIndex(index)) => {
            PREDEFINED_SCRIPTS[index].script.execute()
        }
        Some(dialog::Outcome::ScriptOptions(options)) => {
            let script = Script {
                next_boot_operating_system: Some(options.next_boot_operating_system.into()),
                // TODO: implement it
                next_windows_boot_profile: None,
                next_windows_boot_display: Some(options.next_windows_boot_display.into()),
                #[cfg(windows)]
                switch_to_display: options.switch_display.then_some(SwitchToDisplay::Other),
                reboot_action: options.reboot_action,
            };
            script.execute()
        }
        None => Ok(()),
    }
}

fn execute_script(script: Script) -> Result<()> {
    script.execute()
}

fn show_state() -> Result<()> {
    let provider = StateProvider::new()?;
    let state = provider.get_state()?;

    println!(
        "{}: {}",
        text::operating_system::ON_NEXT_BOOT_DESCRIPTION,
        text::operating_system::value_text(state.next_boot_operating_system)
    );
    println!(
        "{}: {}",
        text::display::ON_NEXT_WINDOWS_BOOT_DESCRIPTION,
        text::display::value_text(state.next_windows_boot_display)
    );
    println!(
        "{}: {}",
        text::profile::ON_NEXT_WINDOWS_BOOT_DESCRIPTION,
        text::profile::next_boot_value_text(
            state
                .next_windows_boot_profile
                .and_then(|id| LabeledProfile::get(id, provider.configs()))
        )
    );
    #[cfg(windows)]
    println!(
        "{}: {}",
        text::profile::CURRENT,
        text::profile::current_value_text(
            state
                .current_profile
                .and_then(|id| LabeledProfile::get(id, provider.configs()))
        )
    );
    #[cfg(windows)]
    println!(
        "{}: {}",
        text::display::CURRENT,
        text::display::value_text(Some(state.current_display))
    );

    Ok(())
}

#[cfg(not(windows))]
fn configure() -> Result<()> {
    host_os::configuration::configure()
}

#[cfg(windows)]
use options_types::Display;
#[cfg(windows)]
fn configure(initial_display: Option<Display>) -> Result<()> {
    host_os::configuration::configure(initial_display)
}

fn show_usage() -> Result<()> {
    let configs = Configs::load(true)?;

    let usage = args::Usage::new(
        configs.get_profile_label_opt(ProfileId::A),
        configs.get_profile_label_opt(ProfileId::B),
    );

    println!("{usage}");
    Ok(())
}
