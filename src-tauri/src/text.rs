use ansi_term::ANSIString;
use ansi_term::Color::{Blue, Green, Red};

use crate::options_types::OptionType;

pub mod operating_system {
    use ansi_term::ANSIString;

    use crate::options_types::OperatingSystem;

    pub const ON_NEXT_BOOT_DESCRIPTION: &str =
        "sistema operacional a ser iniciado na próxima inicialização do computador";

    pub const WAS_UPDATED_TO: &str = "foi atualizado para";

    pub fn value_text(os: Option<OperatingSystem>) -> ANSIString<'static> {
        super::two_values_option_value_text(
            os,
            OperatingSystem::Windows,
            OperatingSystem::Linux,
            "indefinido",
        )
    }
}

pub mod display {
    use ansi_term::ANSIString;

    use crate::options_types::Display;

    pub const ON_NEXT_WINDOWS_BOOT_DESCRIPTION: &str =
        "tela a ser usada na próxima inicialização do Windows";

    pub const CURRENT: &str = "tela atual";

    pub const WAS_UPDATED_TO: &str = "foi atualizada para";

    pub fn value_text(display: Option<Display>) -> ANSIString<'static> {
        super::two_values_option_value_text(display, Display::TV, Display::Monitor, "indefinida")
    }
}

fn two_values_option_value_text<T: OptionType + PartialEq + ToString>(
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
