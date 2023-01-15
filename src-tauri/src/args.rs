use std::env;
use std::error::Error;
use std::fmt::Display;

pub enum ParsedArgs {
    ShowState,
    None,
    Usage,
    Temporary,
}

#[derive(Debug)]
pub struct ArgError {
    message: String,
    arg: String,
}

impl ArgError {
    fn new(message: &str, arg: &str) -> ArgError {
        ArgError {
            message: message.to_string(),
            arg: arg.to_string(),
        }
    }
}
impl Error for ArgError {}
impl Display for ArgError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.message, self.arg)
    }
}

pub fn parse() -> Result<ParsedArgs, ArgError> {
    let mut args = env::args();
    args.next();

    let parsed_args = match args.next() {
        Some(arg) => match &arg[..] {
            "show" => ParsedArgs::ShowState,
            "-h" | "--help" => ParsedArgs::Usage,
            "-" => ParsedArgs::Temporary,
            _ => return unknown_argument_error(&arg),
        },
        None => ParsedArgs::None,
    };

    check_no_more_arguments(&mut args)?;
    Ok(parsed_args)
}

fn check_no_more_arguments(args: &mut env::Args) -> Result<(), ArgError> {
    match args.next() {
        Some(arg) => exceeding_argument_error(&arg),
        None => Ok(()),
    }
}

fn exceeding_argument_error<T>(arg: &str) -> Result<T, ArgError> {
    Err(ArgError::new("Argumento em excesso", arg))
}

fn unknown_argument_error<T>(arg: &str) -> Result<T, ArgError> {
    Err(ArgError::new("Argumento inesperado", arg))
}

pub const USAGE: &str = "\
Usos:
  my-reboot show
    Exibe as opções atuais para inicialização.

  my-reboot -h|--help
    Exibe este conteúdo.";
