use ansi_term::ANSIString;
use ansi_term::Color::{Blue, Green, Red};

use crate::options_types::OptionType;

pub mod operating_system {
    use ansi_term::ANSIString;

    use crate::options_types::OperatingSystem;

    pub const ON_NEXT_BOOT_DESCRIPTION: &str =
        "sistema operacional a ser iniciado na próxima inicialização do computador";

    pub const WAS_UPDATED_TO: &str = "foi atualizado para";

    pub const UNDEFINED: &str = "indefinido";

    pub fn value_text(os: Option<OperatingSystem>) -> ANSIString<'static> {
        super::two_values_option_value_text(
            os,
            OperatingSystem::Windows,
            OperatingSystem::Linux,
            UNDEFINED,
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

    pub const UNDEFINED: &str = "indefinida";

    pub mod switching {
        pub const NOT_SUPPORTED: &str =
            "A troca de tela não é suportada no sistema operacional atual";
        pub const TO: &str = "Trocando de tela para";
        #[cfg(windows)]
        pub const TAKING_TOO_LONG: &str = "A tela não trocou no tempo limite";
        #[cfg(windows)]
        pub const FAILED: &str = "A troca de tela falhou";
        pub const IS_ALREADY_CURRENT: &str = "já é a tela atual";
    }

    pub fn value_text(display: Option<Display>) -> ANSIString<'static> {
        super::two_values_option_value_text(display, Display::TV, Display::Monitor, UNDEFINED)
    }
}

pub mod reboot_action {
    pub const FAILED: &str = "A ação de reinicialização falhou";
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

pub trait Capitalize {
    fn capitalize(self) -> String;
}
impl Capitalize for &str {
    fn capitalize(self) -> String {
        let mut output = String::new();

        let mut chars = self.chars();
        if let Some(ch) = chars.next() {
            output.extend(ch.to_uppercase());
        }
        output.extend(chars);

        output
    }
}

#[cfg(test)]
mod tests {
    use super::Capitalize;

    #[test]
    fn capitalize() {
        assert_eq!("".to_string().capitalize(), "");
        assert_eq!("hello world".to_string().capitalize(), "Hello world");
    }
}
