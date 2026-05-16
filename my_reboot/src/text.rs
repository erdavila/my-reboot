use std::fmt::Write;

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
        super::two_values_option_value_text(os, UNDEFINED)
    }
}

pub mod display {
    use ansi_term::ANSIString;

    use crate::options_types::Display;

    pub const ON_NEXT_WINDOWS_BOOT_DESCRIPTION: &str =
        "tela a ser usada na próxima inicialização do Windows";

    #[cfg(windows)]
    pub const CURRENT: &str = "tela atual";

    pub const WAS_UPDATED_TO: &str = "foi atualizada para";

    pub const UNDEFINED: &str = "indefinida";

    #[cfg(windows)]
    pub mod switching {
        pub const TO: &str = "Trocando de tela para";
        pub const TAKING_TOO_LONG: &str = "A tela não trocou no tempo limite";
        pub const FAILED: &str = "A troca de tela falhou";
        pub const IS_ALREADY_CURRENT: &str = "já é a tela atual";
    }

    pub fn value_text(display: Option<Display>) -> ANSIString<'static> {
        super::two_values_option_value_text(display, UNDEFINED)
    }
}

pub mod reboot_action {
    pub const FAILED: &str = "A ação de reinicialização falhou";
}

fn two_values_option_value_text<T: OptionType + PartialEq + ToString>(
    current_value: Option<T>,
    undefined_text: &str,
) -> ANSIString<'static> {
    let (color, text) = match current_value {
        Some(current_value) => {
            let [value1, value2] = T::values();
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

pub(crate) struct Capitalized<T>(pub(crate) T);
impl<T: std::fmt::Display> std::fmt::Display for Capitalized<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        struct Adapter<'a, 'b> {
            inner: &'a mut std::fmt::Formatter<'b>,
            uppercased_first_char: bool,
        }
        impl Adapter<'_, '_> {
            fn write_uppercased_char(&mut self, c: char) -> std::fmt::Result {
                for c in c.to_uppercase() {
                    self.inner.write_char(c)?;
                }
                self.uppercased_first_char = true;
                Ok(())
            }
        }
        impl std::fmt::Write for Adapter<'_, '_> {
            fn write_str(&mut self, s: &str) -> std::fmt::Result {
                if self.uppercased_first_char {
                    self.inner.write_str(s)
                } else if let Some(first_char) = s.chars().next() {
                    self.write_uppercased_char(first_char)?;

                    // Remaining chars.
                    let i = s.ceil_char_boundary(1);
                    self.inner.write_str(&s[i..])
                } else {
                    Ok(())
                }
            }

            fn write_char(&mut self, c: char) -> std::fmt::Result {
                if self.uppercased_first_char {
                    self.inner.write_char(c)
                } else {
                    self.write_uppercased_char(c)
                }
            }
        }

        let mut adapter = Adapter {
            inner: f,
            uppercased_first_char: false,
        };
        write!(adapter, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capitalized_write_str_empty_write_str() {
        let display = std::fmt::from_fn(|f| {
            f.write_str("")?;
            f.write_str("óbvio")
        });

        assert_eq!(Capitalized(display).to_string(), "Óbvio");
    }

    #[test]
    fn capitalized_write_str_twice() {
        let display = std::fmt::from_fn(|f| {
            f.write_str("óbvio-")?;
            f.write_str("óbvio")
        });

        assert_eq!(Capitalized(display).to_string(), "Óbvio-óbvio");
    }

    #[test]
    fn capitalized_write_char_write_str() {
        let display = std::fmt::from_fn(|f| {
            f.write_char('ó')?;
            f.write_str("bvio")
        });

        assert_eq!(Capitalized(display).to_string(), "Óbvio");
    }

    #[test]
    fn capitalized_write_char_twice() {
        let display = std::fmt::from_fn(|f| {
            f.write_char('ó')?;
            f.write_char('b')
        });

        assert_eq!(Capitalized(display).to_string(), "Ób");
    }
}
