use std::fmt::{Display, Write};

use ansi_term::ANSIString;
use ansi_term::Color::{Blue, Green, Red};

use crate::options_types::Values;

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

pub(crate) mod profile {
    use ansi_term::ANSIString;

    use crate::options_types::LabeledProfile;

    pub(crate) const ON_NEXT_WINDOWS_BOOT_DESCRIPTION: &str =
        "perfil a ser usado na próxima inicialização do Windows";

    #[cfg(any(windows, test))]
    pub(crate) const SWITCH_DESCRIPTION: &str = "troca de perfil";

    pub(crate) const WAS_UPDATED_TO: &str = "foi atualizado para";

    #[cfg(windows)]
    pub(crate) const CURRENT: &str = "perfil atual";

    pub(crate) const UNDEFINED: &str = "indefinido";

    #[cfg(windows)]
    const UNRECOGNIZED: &str = "não reconhecido";

    #[cfg(windows)]
    pub(crate) mod switching {
        pub(crate) const TO: &str = "Trocando de perfil para";
        pub(crate) const TAKING_TOO_LONG: &str = "O perfil não trocou no tempo limite";
        pub(crate) const IS_ALREADY_CURRENT: &str = "já é o perfil atual";
    }

    pub(crate) fn next_boot_value_text(
        labeled_profile: Option<LabeledProfile>,
    ) -> ANSIString<'static> {
        value_text(labeled_profile, UNDEFINED)
    }

    #[cfg(windows)]
    pub(crate) fn current_value_text(
        labeled_profile: Option<LabeledProfile>,
    ) -> ANSIString<'static> {
        value_text(labeled_profile, UNRECOGNIZED)
    }

    fn value_text(
        labeled_profile: Option<LabeledProfile>,
        undefined_text: &str,
    ) -> ANSIString<'static> {
        let profile_label = labeled_profile.map(|lp| (lp.profile_id(), lp.to_string()));
        super::two_values_text(profile_label, undefined_text)
    }
}

pub mod reboot_action {
    pub(crate) const ACTION_DESCRIPTION: &str = "ação";
    pub(crate) const UNDEFINED: &str = "indefinida";
    pub const FAILED: &str = "A ação de reinicialização falhou";
}

fn two_values_option_value_text<T: Values + PartialEq + ToString>(
    current_value: Option<T>,
    undefined_text: &str,
) -> ANSIString<'static> {
    let current_value = current_value.map(|value| (value, value.to_string()));
    two_values_text(current_value, undefined_text)
}

fn two_values_text<T: Values + PartialEq>(
    current_value: Option<(T, String)>,
    undefined_text: &str,
) -> ANSIString<'static> {
    let (color, text) = match current_value {
        Some((current_value, text)) => {
            let [value1, value2] = T::values();
            let color = if current_value == value1 {
                Blue
            } else if current_value == value2 {
                Green
            } else {
                unimplemented!()
            };
            (color, text)
        }
        None => (Red, undefined_text.to_string()),
    };
    color.bold().paint(text)
}

pub(crate) struct Capitalized<T>(pub(crate) T);
impl<T: Display> Display for Capitalized<T> {
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
