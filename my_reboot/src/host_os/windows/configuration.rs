use anyhow::Result;
use is_windows_11_or_greater::is_windows_11_or_greater;

use crate::{
    configs::ConfigsWriter,
    host_os::{
        WindowsCurrentDisplayHandler, windows::get_active_display_id::get_active_display_id,
    },
    options_types::{Display, OptionType},
};

pub fn configure(initial_display: Option<Display>) -> Result<()> {
    if let Some(initial_display) = initial_display {
        let display_switch_args = if is_windows_11_or_greater() {
            ["1", "4"]
        } else {
            ["/internal", "/external"]
        };

        let initial_display_device_id = get_active_display_id();
        let other_display = match initial_display {
            Display::Monitor => Display::TV,
            Display::TV => Display::Monitor,
        };

        println!("Trocando de tela...");
        const WAIT_SECONDS: u64 = 5;
        let switched = WindowsCurrentDisplayHandler::execute_display_switch(
            display_switch_args[0],
            WAIT_SECONDS,
        )?;
        let (initial_display_switch_arg, other_display_device_id, other_display_switch_arg) =
            if switched {
                (
                    display_switch_args[1],
                    get_active_display_id(),
                    display_switch_args[0],
                )
            } else {
                WindowsCurrentDisplayHandler::execute_display_switch(
                    display_switch_args[1],
                    WAIT_SECONDS,
                )?;
                (
                    display_switch_args[0],
                    get_active_display_id(),
                    display_switch_args[1],
                )
            };

        println!("Voltando para a tela inicial...");
        WindowsCurrentDisplayHandler::execute_display_switch(
            initial_display_switch_arg,
            WAIT_SECONDS,
        )?;

        let mut configs = ConfigsWriter::load(false)?;
        configs.set_device_id(initial_display, &initial_display_device_id);
        configs.set_device_id(other_display, &other_display_device_id);
        configs.set_display_switch_arg(initial_display, initial_display_switch_arg);
        configs.set_display_switch_arg(other_display, other_display_switch_arg);
        println!("Salvando configurações...");
        configs.save()?;

        println!("Configuração finalizada.");
        Ok(())
    } else {
        println!(
            "\
Execute:
  my-reboot configure TELA
onde TELA é a tela atual ({}).

Será testada a troca de telas. A configuração termina ao retornar para a tela inicial.",
            Display::values()
                .into_iter()
                .map(|d| format!("\"{}\"", d.to_option_string()))
                .collect::<Vec<_>>()
                .join(" ou ")
        );
        Ok(())
    }
}
