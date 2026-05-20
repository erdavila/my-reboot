mod errors;
mod script_args;

use std::env;
use std::fmt::Display;

use anyhow::Result;

use self::errors::ArgError;
use crate::dialog::Mode;
use crate::host_os::PREDEFINED_SCRIPTS;
use crate::options_types::{LabeledProfile, OptionType as _, ProfileId};
use crate::script::Script;

pub enum ParsedArgs {
    Dialog(Mode),
    ShowState,
    Script(Script),
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
                let index = parse_script_args(&mut args)?;
                ParsedArgs::Script(PREDEFINED_SCRIPTS[index].script)
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

fn parse_script_args(args: &mut env::Args) -> Result<usize, ArgError> {
    match args.next() {
        Some(arg) => match arg.parse::<usize>() {
            Ok(number) if number >= 1 && number <= PREDEFINED_SCRIPTS.len() => Ok(number - 1),
            _ => Err(ArgError::new(
                &format!(
                    "Número inválido de script para o sistema operacional atual (mín: 1; máx: {})",
                    PREDEFINED_SCRIPTS.len()
                ),
                &arg,
            )),
        },
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let [profile_a, profile_b] = ProfileId::values().map(|id| {
            std::fmt::from_fn(move |f| {
                if let Ok(labels) = &self.profile_labels {
                    write!(f, "{}", LabeledProfile::new(id, &labels[id as usize]))
                } else {
                    write!(f, "{id} (*)")
                }
            })
        });

        write!(
            f,
            "\
Usos:
  my-reboot [dialog]
    Exibe diálogo básico.

  my-reboot dialog -x
    Exibe diálogo avançado.

  my-reboot (SO | PERFIL | "
        )?;

        #[cfg(windows)]
        write!(f, "TROCA-DE-PERFIL | ")?;

        write!(
            f,
            "AÇÃO)+
    SO pode ser:
      [os:]windows - Inicia Windows na próxima inicialização do computador.
      [os:]linux - Inicia Linux na próxima inicialização do computador.
      os:unset - Deixa o Grub decidir o S.O. na próxima inicialização do computador.

    PERFIL pode ser:
      profile:a - Usa o perfil {profile_a} na próxima inicialização do Windows.
      profile:b - Usa o perfil {profile_b} na próxima inicialização do Windows.
      profile:unset - Deixa o Windows decidir o perfil na próxima inicialização.

    ",
        )?;

        #[cfg(windows)]
        write!(
            f,
            "TROCA-DE-PERFIL pode ser:
      switch[:other] - Troca para o outro perfil.
      switch:a - Troca para o perfil {profile_a}.
      switch:b - Troca para o perfil {profile_b}.
      switch:saved - Troca para o perfil definido para ser usado na próxima inicialização do Windows.

    ",
        )?;

        write!(
            f,
            "AÇÃO pode ser:
      reboot - Reinicia o computador.
      shutdown - Desliga o computador.

  my-reboot show
    Exibe as opções atuais para inicialização.

  my-reboot script NÚMERO
    Executa o script correspondente às ações disponíveis no diálogo básico do S.O. atual.

  my-reboot configure
    Configura. Deve ser executado no Linux e no Windows ao menos uma vez.

  my-reboot -h|--help
    Exibe este conteúdo."
        )?;

        if let Err(e) = &self.profile_labels {
            write!(
                f,
                "


(*): {e}"
            )?;
        }

        Ok(())
    }
}
