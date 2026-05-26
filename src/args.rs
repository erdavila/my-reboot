mod errors;
pub(crate) mod script_args;

use std::env;
use std::fmt::Display;
use std::marker::PhantomData;
use std::num::NonZeroUsize;

use anyhow::Result;

use self::errors::ArgError;
#[cfg(windows)]
use crate::args::script_args::SWITCH_TO_PROFILE_PREFIX;
use crate::args::script_args::{
    NEXT_BOOT_OPERATING_SYSTEM_PREFIX, NEXT_WINDOWS_BOOT_PROFILE_PREFIX,
};
use crate::dialog::Mode;
use crate::options_types::{
    LabeledProfile, OperatingSystem, ProfileId, SerializeToString, Values as _,
};
use crate::script::{Script, SetOrUnset};

pub enum ParsedArgs {
    Dialog(Mode),
    ShowState,
    Script(Script),
    PredefinedScriptNumber(NonZeroUsize),
    Configure,
    Usage,
}

pub fn parse() -> Result<ParsedArgs, ArgError> {
    let mut args = env::args();
    args.next();

    let parsed_args = match args.next() {
        Some(arg) => match &arg[..] {
            "dialog" => {
                let mode = parse_dialog_args(&mut args)?;
                ParsedArgs::Dialog(mode)
            }
            "show" => ParsedArgs::ShowState,
            "script" => {
                let number = parse_script_args(&mut args)?;
                ParsedArgs::PredefinedScriptNumber(number)
            }
            "configure" => ParsedArgs::Configure,
            "-h" | "--help" => ParsedArgs::Usage,
            _ => match script_args::parse(&arg, &mut args)? {
                Some(script) => ParsedArgs::Script(script),
                None => return errors::unknown_argument_error(&arg),
            },
        },
        None => ParsedArgs::Dialog(Mode::Basic),
    };

    errors::check_no_more_arguments(&mut args)?;
    Ok(parsed_args)
}

fn parse_dialog_args(args: &mut env::Args) -> Result<Mode, ArgError> {
    match args.next() {
        None => Ok(Mode::Basic),
        Some(arg) if arg == "-x" => Ok(Mode::Advanced),
        Some(arg) => errors::unknown_argument_error(&arg),
    }
}

fn parse_script_args(args: &mut env::Args) -> Result<NonZeroUsize, ArgError> {
    match args.next() {
        Some(arg) => arg
            .parse()
            .map_err(|e| ArgError::new(&format!("Número inválido de script {arg:?}: {e}"), &arg)),
        None => errors::missing_argument_error("NÚMERO"),
    }
}

pub(crate) struct Usage {
    profile_labels: Result<[String; 2]>,
}
impl Usage {
    pub(crate) fn new(profile_labels: Result<[String; 2]>) -> Self {
        Self { profile_labels }
    }
}
impl Display for Usage {
    #[expect(clippy::too_many_lines)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let profiles = ProfileId::values().map(|id| {
            let display = std::fmt::from_fn(move |f| {
                if let Ok(labels) = &self.profile_labels {
                    write!(f, "{}", LabeledProfile::new(id, &labels[id as usize]))
                } else {
                    write!(f, "{id} (*)")
                }
            });
            (id, display)
        });

        let mut f = UsageWriter::new(f);

        f.write_block("Usos:", |f| {
            f.write_block("my-reboot [dialog]", |f| {
                f.write("Exibe diálogo básico.")?;
                f.write("")
            })?;

            f.write_block("my-reboot dialog -x", |f| {
                f.write("Exibe diálogo avançado.")?;
                f.write("")
            })?;

            f.write_block(std::fmt::from_fn(|f| {
                write!(f, "my-reboot (SO | PERFIL | ")?;
                #[cfg(windows)]
                write!(f, "TROCA-DE-PERFIL | ")?;
                write!(f, "AÇÃO)+")
            }), |f| {
                f.write_block("SO pode ser:", |f| {
                    let pouf = PrefixedOptionUsageFormatter::<SetOrUnset<OperatingSystem>>::new(NEXT_BOOT_OPERATING_SYSTEM_PREFIX);
                    for os in OperatingSystem::values() {
                        f.write(pouf.format(
                            PrefixedOptionUsageFormat::OptionalPrefix,
                            os,
                            format_args!("Inicia {os} na próxima inicialização do computador."),
                        ))?;
                    }
                    f.write(pouf.format(
                        PrefixedOptionUsageFormat::Full,
                        SetOrUnset::Unset,
                        "Deixa o Grub decidir o S.O. na próxima inicialização do computador.",
                    ))?;
                    f.write("")
                })?;

                f.write_block("PERFIL pode ser:", |f| {
                    let pouf = PrefixedOptionUsageFormatter::<SetOrUnset<ProfileId>>::new(NEXT_WINDOWS_BOOT_PROFILE_PREFIX);
                    for (id, profile) in &profiles {
                        f.write(pouf.format(
                            PrefixedOptionUsageFormat::Full,
                            *id,
                            format_args!("Usa o perfil {profile} na próxima inicialização do {}.", OperatingSystem::Windows),
                        ))?;
                    }
                    f.write(pouf.format(
                        PrefixedOptionUsageFormat::Full,
                        SetOrUnset::Unset,
                        format_args!("Deixa o {} decidir o perfil na próxima inicialização.", OperatingSystem::Windows),
                    ))?;
                    f.write("")
                })?;

                #[cfg(windows)]
                f.write_block("TROCA-DE-PERFIL pode ser:", |f| {
                    use crate::script::SwitchToProfile;
                    let pouf = PrefixedOptionUsageFormatter::<SwitchToProfile>::new(SWITCH_TO_PROFILE_PREFIX);
                    f.write(pouf.format(PrefixedOptionUsageFormat::OptionalValue, SwitchToProfile::Other, "Troca para o outro perfil."))?;
                    for (id, profile) in &profiles {
                        f.write(pouf.format(
                            PrefixedOptionUsageFormat::Full,
                            *id,
                            format_args!("Troca para o perfil {profile}."),
                        ))?;
                    }
                    f.write(pouf.format(
                        PrefixedOptionUsageFormat::Full,
                        SwitchToProfile::Saved,
                        format_args!("Troca para o perfil definido para ser usado na próxima inicialização do {}.", OperatingSystem::Windows),
                    ))?;
                    f.write("")
                })?;

                f.write_block("AÇÃO pode ser:", |f| {
                    f.write("reboot - Reinicia o computador.")?;
                    f.write("shutdown - Desliga o computador.")?;
                    f.write("")
                })
            })?;

            f.write_block("my-reboot show", |f| {
                f.write("Exibe as opções atuais para inicialização.")?;
                f.write("")
            })?;

            f.write_block("my-reboot script NÚMERO", |f| {
                f.write("Executa o script correspondente às ações disponíveis no diálogo básico do S.O. atual.")?;
                f.write("")
            })?;

            f.write_block("my-reboot configure", |f| {
                f.write(format_args!("Configura. Deve ser executado no {} e no {} ao menos uma vez.", OperatingSystem::Linux, OperatingSystem::Windows))?;
                f.write("")
            })?;

            f.write_block("my-reboot -h|--help", |f| {
                f.write("Exibe este conteúdo.")
            })
        })?;

        if let Err(e) = &self.profile_labels {
            f.write("")?;
            f.write("")?;
            f.write(format_args!("(*): {e}"))?;
        }

        Ok(())
    }
}

struct UsageWriter<'a, 'b> {
    f: &'a mut std::fmt::Formatter<'b>,
    indent: usize,
}
impl<'a, 'b> UsageWriter<'a, 'b> {
    fn new(f: &'a mut std::fmt::Formatter<'b>) -> Self {
        Self { f, indent: 0 }
    }

    fn write_block(
        &mut self,
        display: impl Display,
        block: impl FnOnce(&mut Self) -> std::fmt::Result,
    ) -> std::fmt::Result {
        self.write_indent()?;
        writeln!(self.f, "{display}")?;

        self.indent += 1;
        block(self)?;
        self.indent -= 1;

        Ok(())
    }

    fn write(&mut self, display: impl Display) -> std::fmt::Result {
        self.write_block(display, |_| Ok(()))
    }

    fn write_indent(&mut self) -> std::fmt::Result {
        for _ in 0..self.indent {
            write!(self.f, "  ")?;
        }
        Ok(())
    }
}

enum PrefixedOptionUsageFormat {
    #[cfg(windows)]
    OptionalValue,
    OptionalPrefix,
    Full,
}

struct PrefixedOptionUsageFormatter<T> {
    prefix: &'static str,
    phantom: PhantomData<T>,
}
impl<T: SerializeToString> PrefixedOptionUsageFormatter<T> {
    fn new(prefix: &'static str) -> Self {
        Self {
            prefix,
            phantom: PhantomData,
        }
    }

    fn format<'a>(
        &'a self,
        format: PrefixedOptionUsageFormat,
        value: impl Into<T> + 'a,
        description: impl Display + 'a,
    ) -> impl Display + 'a {
        let value = value.into().serialize_to_string();
        std::fmt::from_fn(move |f| {
            match format {
                #[cfg(windows)]
                PrefixedOptionUsageFormat::OptionalValue => write!(f, "{}[:{value}]", self.prefix)?,
                PrefixedOptionUsageFormat::OptionalPrefix => {
                    write!(f, "[{}:]{value}", self.prefix)?;
                }
                PrefixedOptionUsageFormat::Full => write!(f, "{}:{value}", self.prefix)?,
            }
            write!(f, " - {description}")
        })
    }
}
