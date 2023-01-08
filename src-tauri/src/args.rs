use std::env;
use std::error::Error;
use std::fmt::Display;

pub enum ParsedArgs {
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
    fn new(message: &str, arg: String) -> ArgError {
        ArgError {
            message: message.to_string(),
            arg,
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
            "-h" | "--help" => ParsedArgs::Usage,
            "-" => ParsedArgs::Temporary,
            _ => return Err(ArgError::new("Argumento inesperado", arg)),
        },
        None => ParsedArgs::None,
    };

    Ok(parsed_args)
}

pub const USAGE: &str = "\
Usos:
  my-reboot -h|--help
    Exibe este conteúdo.";
