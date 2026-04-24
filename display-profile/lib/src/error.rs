use std::fmt::Display;
use std::string::FromUtf16Error;

use windows::Win32::Foundation::WIN32_ERROR;

#[derive(Debug)]
pub enum Error {
    WindowsApiError {
        function: &'static str,
        code: WIN32_ERROR,
        message: String,
    },
    FromUtf16Error(FromUtf16Error),
    Custom(String),
}

impl From<FromUtf16Error> for Error {
    fn from(value: FromUtf16Error) -> Self {
        Error::FromUtf16Error(value)
    }
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::WindowsApiError {
                function,
                code,
                message,
            } => write!(
                f,
                "Windows API function {function} failed with error {} - {message}",
                code.0
            ),
            Error::FromUtf16Error(err) => write!(f, "failed to convert from UTF-16: {err}"),
            Error::Custom(message) => write!(f, "{message}"),
        }
    }
}
