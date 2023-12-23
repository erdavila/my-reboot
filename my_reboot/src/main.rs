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

use crate::args::ParsedArgs;
use crate::state::StateProvider;

use anyhow::{Context, Result};
use script::Script;

fn main() -> Result<()> {
    let args = args::parse()
        .with_context(|| "Argumentos inválidos.\nPara ajuda, execute: my-reboot --help")?;

    match args {
        ParsedArgs::Dialog => show_dialog(),
        ParsedArgs::Script(script) => execute_script(script),
        ParsedArgs::ShowState => show_state(),
        ParsedArgs::Usage => {
            println!("{}", args::USAGE);
            Ok(())
        }
    }
}

fn show_dialog() -> Result<()> {
    dialog::show_advanced()
}

fn execute_script(script: Script) -> Result<()> {
    script.execute()
}

fn show_state() -> Result<()> {
    let provider = StateProvider::new()?;
    let state = provider.get_state();

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
    if state.current_display.is_some() {
        println!(
            "{}: {}",
            text::display::CURRENT,
            text::display::value_text(state.current_display)
        );
    }

    Ok(())
}
