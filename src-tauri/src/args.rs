mod errors;
mod script_args;

use std::env;

use crate::script::Script;

use self::errors::ArgError;

pub enum ParsedArgs {
    ShowState,
    Script(Script),
    None,
    Usage,
    Temporary,
}

pub fn parse() -> Result<ParsedArgs, ArgError> {
    let mut args = env::args();
    args.next();

    let parsed_args = match args.next() {
        Some(arg) => match &arg[..] {
            "show" => ParsedArgs::ShowState,
            "-h" | "--help" => ParsedArgs::Usage,
            "-" => ParsedArgs::Temporary,
            _ => match script_args::parse(&arg, &mut args)? {
                Some(script) => ParsedArgs::Script(script),
                None => return errors::unknown_argument_error(&arg),
            },
        },
        None => ParsedArgs::None,
    };

    errors::check_no_more_arguments(&mut args)?;
    Ok(parsed_args)
}

pub const USAGE: &str = "\
Usos:
  my-reboot SO
    SO poder ser:
      [os:]windows - Inicia Windows na próxima inicialização do computador.
      [os:]linux - Inicia Linux na próxima inicialização do computador.
      os:unset - Deixa o Grub decidir o S.O. na próxima inicialização do computador.

  my-reboot show
    Exibe as opções atuais para inicialização.

  my-reboot -h|--help
    Exibe este conteúdo.";
