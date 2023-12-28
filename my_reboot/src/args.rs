mod errors;
mod script_args;

use std::env;

use crate::dialog::Mode;
use crate::host_os::PREDEFINED_SCRIPTS;
#[cfg(windows)]
use crate::options_types::Display;
use crate::script::Script;

use self::errors::ArgError;

pub enum ParsedArgs {
    Dialog(Mode),
    ShowState,
    Script(Script),
    #[cfg(not(windows))]
    Configure,
    #[cfg(windows)]
    Configure {
        initial_display: Option<Display>,
    },
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
            #[cfg(not(windows))]
            "configure" => ParsedArgs::Configure,
            #[cfg(windows)]
            "configure" => {
                let initial_display = parse_configure_args(&mut args)?;
                ParsedArgs::Configure { initial_display }
            }
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

#[cfg(windows)]
fn parse_configure_args(args: &mut env::Args) -> Result<Option<Display>, ArgError> {
    use crate::options_types::OptionType;

    if let Some(arg) = args.next() {
        let display = Display::values()
            .into_iter()
            .find(|display| display.to_option_string() == arg);
        if display.is_some() {
            Ok(display)
        } else {
            errors::unknown_argument_error(&arg)
        }
    } else {
        Ok(None)
    }
}

pub const USAGE: &str = "\
Usos:
  my-reboot [dialog]
    Exibe diálogo básico.

  my-reboot dialog -x
    Exibe diálogo avançado.

  my-reboot (SO | TELA | TROCA-DE-TELA | AÇÃO)+
    SO poder ser:
      [os:]windows - Inicia Windows na próxima inicialização do computador.
      [os:]linux - Inicia Linux na próxima inicialização do computador.
      os:unset - Deixa o Grub decidir o S.O. na próxima inicialização do computador.

    TELA poder ser:
      [display:]monitor - Usa o monitor na próxima inicialização do Windows.
      [display:]tv - Usa a TV na próxima inicialização do Windows.
      display:unset - Deixa o Windows decidir a tela na próxima inicialização do Windows.

    TROCA-DE-TELA poder ser (somente no Windows):
      switch[:other] - Troca para a outra tela.
      switch:monitor - Troca para o monitor.
      switch:tv - Troca para a TV.
      switch:saved - Troca para a tela definida para ser usada na próxima inicialização do Windows.

    AÇÃO poder ser:
      reboot - Reinicia o computador.
      shutdown - Desliga o computador.

  my-reboot show
    Exibe as opções atuais para inicialização.

  my-reboot script NÚMERO
    Executa o script correspondente às ações disponíveis no diálogo básico do S.O. atual.

  my-reboot configure
    Configura. Deve ser executado no Linux e no Windows ao menos uma vez.

  my-reboot -h|--help
    Exibe este conteúdo.";
