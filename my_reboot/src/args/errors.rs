use std::env::Args;
use std::error::Error;
use std::fmt::Display;

#[derive(Debug)]
pub struct ArgError {
    message: String,
    arg: String,
}
impl ArgError {
    pub fn new(message: &str, arg: &str) -> ArgError {
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

pub fn check_no_more_arguments(args: &mut Args) -> Result<(), ArgError> {
    match args.next() {
        Some(arg) => exceeding_argument_error(&arg),
        None => Ok(()),
    }
}

fn exceeding_argument_error<T>(arg: &str) -> Result<T, ArgError> {
    Err(ArgError::new("Argumento em excesso", arg))
}

pub fn unknown_argument_error<T>(arg: &str) -> Result<T, ArgError> {
    Err(ArgError::new("Argumento inesperado", arg))
}

pub fn missing_argument_error<T>(name: &str) -> Result<T, ArgError> {
    Err(ArgError::new("Argumento faltando", name))
}
