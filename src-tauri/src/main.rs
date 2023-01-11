#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod args;
mod file_content_as_hash_map;
mod grubenv;
mod host_os;
mod properties;

use std::env;
use std::error::Error;

use crate::args::ParsedArgs;
use crate::grubenv::Grubenv;
use crate::properties::Properties;

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
        ParsedArgs::Temporary => {
            let grubenv = Grubenv::load().unwrap();
            let saved_entry = grubenv.get("saved_entry").unwrap();
            println!("saved_entry = {saved_entry}");

            let properties = Properties::load("my-reboot-options.properties", true).unwrap();
            let windows_display = properties.get("windows.display").unwrap();
            println!("windows_display = {windows_display}");

            if false {
                let mut grubenv = grubenv;
                grubenv.set("dummy", "dummy");
                grubenv.save()?;

                let mut properties = properties;
                properties.set("windows.display", "tv");
                properties.save().unwrap();
            }
        }
        ParsedArgs::Usage => println!("{}", args::USAGE),
    }

    Ok(())
}
