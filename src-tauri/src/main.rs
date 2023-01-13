#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod args;
mod host_os;

use std::env;
use std::error::Error;

use crate::args::ParsedArgs;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() -> Result<(), Box<dyn Error>> {
    match args::parse()? {
        ParsedArgs::None => {
            host_os::enumerate_display_devices();

            tauri::Builder::default()
                .invoke_handler(tauri::generate_handler![greet])
                .run(tauri::generate_context!())
                .expect("error while running tauri application");
        }
        ParsedArgs::Usage => println!("{}", args::USAGE),
    }

    Ok(())
}
