#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod args;
mod configs;
mod file_content_as_hash_map;
mod grubenv;
mod host_os;
mod options_types;
mod properties;
mod script;
mod state;
mod text;

use crate::args::ParsedArgs;
use crate::properties::Properties;
use crate::state::StateProvider;
use crate::text::NEXT_BOOT_OPERATING_SYSTEM_SENTENCE;
use crate::text::NEXT_WINDOWS_BOOT_DISPLAY_SENTENCE;

use anyhow::{Context, Result};
use script::Script;
use std::env;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() -> Result<()> {
    let args = args::parse()
        .with_context(|| "Argumentos inválidos.\nPara ajuda, execute: my-reboot --help")?;
    match args {
        ParsedArgs::Script(script) => execute_script(script),
        ParsedArgs::ShowState => show_state(),
        ParsedArgs::None => {
            tauri::Builder::default()
                .invoke_handler(tauri::generate_handler![greet])
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
        ParsedArgs::Temporary => {
            if false {
                let mut properties =
                    Properties::load("my-reboot-options.properties", true).unwrap();
                properties.set("windows.display", "tv");
                properties.save().unwrap();
            }
        }
        ParsedArgs::Usage => println!("{}", args::USAGE),
    }

    Ok(())
}

fn execute_script(script: Script) {
    script.execute();
}

fn show_state() {
    let provider = StateProvider::new().unwrap();
    let state = provider.get_state();

    println!(
        "{NEXT_BOOT_OPERATING_SYSTEM_SENTENCE}: {}",
        text::operating_system_text(state.next_boot_operating_system)
    );
    println!(
        "{NEXT_WINDOWS_BOOT_DISPLAY_SENTENCE}: {}",
        text::display_text(state.next_windows_boot_display)
    );
    if state.current_display.is_some() {
        println!("Tela atual: {}", text::display_text(state.current_display));
    }
}
