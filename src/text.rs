use std::fmt::{Display, Write};

use ansi_term::ANSIString;
use ansi_term::Color::{Blue, Green, Red};
#[cfg(windows)]
use display_profile_lib::{Profile, Rotation};

use crate::options_types::{SerializeToString as _, Values};
use crate::persist::configs::{PredefinedScript, ProfileLabel};
use crate::text::indented_block_writer::WriteFmt;

mod indented_block_writer;
pub(crate) use indented_block_writer::IndentedBlockWriter;

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

    use crate::options_types::ProfileId;
    use crate::text::Quoted;

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
        id_and_label: Option<(ProfileId, &str)>,
    ) -> ANSIString<'static> {
        value_text(id_and_label, UNDEFINED)
    }

    #[cfg(windows)]
    pub(crate) fn current_value_text(
        id_and_label: Option<(ProfileId, &str)>,
    ) -> ANSIString<'static> {
        value_text(id_and_label, UNRECOGNIZED)
    }

    fn value_text(
        id_and_label: Option<(ProfileId, &str)>,
        undefined_text: &str,
    ) -> ANSIString<'static> {
        let labeled_profile = id_and_label.map(|(id, label)| (id, labeled_profile(id, label)));
        super::two_values_text(labeled_profile, undefined_text)
    }

    pub(crate) fn labeled_profile(id: ProfileId, label: &str) -> String {
        format!("{} ({id})", Quoted(label))
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

pub(crate) trait IndentedBlockWriterExt {
    type Error;

    #[cfg(windows)]
    fn write_profile_summary(
        &mut self,
        header: impl Display,
        profile: &Profile,
    ) -> Result<(), Self::Error>;

    fn write_predefined_scripts(
        &mut self,
        scripts: &[PredefinedScript],
        profile_label: &impl ProfileLabel,
    ) -> Result<(), Self::Error>;
}
impl<T: WriteFmt> IndentedBlockWriterExt for IndentedBlockWriter<T> {
    type Error = T::Error;

    #[cfg(windows)]
    fn write_profile_summary(
        &mut self,
        header: impl Display,
        profile: &Profile,
    ) -> Result<(), Self::Error> {
        self.write_block(header, |w| {
            for monitor in profile {
                w.write({
                    std::fmt::from_fn(|f| {
                        write!(
                            f,
                            "{}: {}x{}; {:.2}Hz; em {},{}",
                            monitor.friendly_device_name,
                            monitor.dimensions.width,
                            monitor.dimensions.height,
                            f64::from(monitor.refresh_rate.numerator)
                                / f64::from(monitor.refresh_rate.denominator),
                            monitor.position.x,
                            monitor.position.y,
                        )?;

                        let rotation = match monitor.rotation {
                            Rotation::IDENTITY => None,
                            Rotation::ROTATE90 => Some(90),
                            Rotation::ROTATE180 => Some(180),
                            Rotation::ROTATE270 => Some(270),
                        };
                        if let Some(rotation) = rotation {
                            write!(f, "; rotação de {rotation}°")?;
                        }

                        Ok(())
                    })
                })?;
            }
            Ok(())
        })
    }

    fn write_predefined_scripts(
        &mut self,
        scripts: &[PredefinedScript],
        profile_label: &impl ProfileLabel,
    ) -> Result<(), Self::Error> {
        for (i, predef_script) in scripts.iter().enumerate() {
            let number = i + 1;

            let label = predef_script.resolve_label(profile_label);

            self.write_block(format_args!("{number}: '{label}'"), |w| {
                macro_rules! print_option {
                    ($name:ident) => {
                        if let Some(value) = predef_script.script.$name {
                            w.write(format_args!(
                                "{}: {}",
                                stringify!($name),
                                value.serialize_to_string()
                            ))?;
                        }
                    };
                }
                print_option!(next_boot_operating_system);
                print_option!(next_windows_boot_profile);
                print_option!(switch_to_profile);
                print_option!(reboot_action);
                w.write("")
            })?;
        }
        Ok(())
    }
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

pub(crate) struct Quoted<T>(pub(crate) T);
impl<T: Display> Display for Quoted<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\"{}\"", self.0)
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
