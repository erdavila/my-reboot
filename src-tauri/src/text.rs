use ansi_term::ANSIString;
use ansi_term::Color::{Blue, Green, Red};

use crate::options_types::{Display, OperatingSystem, OptionType};

pub const NEXT_BOOT_OPERATING_SYSTEM_SENTENCE: &str =
    "Sistema operacional a ser iniciado na próxima inicialização do computador";
pub const NEXT_WINDOWS_BOOT_DISPLAY_SENTENCE: &str =
    "Tela a ser usada na próxima inicialização do Windows";

pub fn operating_system_text(os: Option<OperatingSystem>) -> ANSIString<'static> {
    two_values_option_option_text(
        os,
        OperatingSystem::Windows,
        OperatingSystem::Linux,
        "indefinido",
    )
}

pub fn display_text(display: Option<Display>) -> ANSIString<'static> {
    two_values_option_option_text(display, Display::TV, Display::Monitor, "indefinida")
}

fn two_values_option_option_text<T: OptionType + PartialEq + ToString>(
    current_value: Option<T>,
    value1: T,
    value2: T,
    undefined_text: &str,
) -> ANSIString<'static> {
    let (color, text) = match current_value {
        Some(current_value) => {
            let color = if current_value == value1 {
                Blue
            } else if current_value == value2 {
                Green
            } else {
                unimplemented!()
            };
            (color, current_value.to_string())
        }
        None => (Red, undefined_text.to_string()),
    };
    color.bold().paint(text)
}
