use windows::Win32::Foundation::{
    ERROR_ACCESS_DENIED, ERROR_GEN_FAILURE, ERROR_INSUFFICIENT_BUFFER, ERROR_INVALID_PARAMETER,
    ERROR_NOT_SUPPORTED, ERROR_SUCCESS, WIN32_ERROR,
};

use crate::Result;
use crate::error::Error;

macro_rules! codes {
    (
        $(
            ( $code:ident, $text:literal ),
        )*
    ) => {
        &[
            $(
                ($code, stringify!($code), $text),
            )*
        ]
    };
}

static CODES: &[(WIN32_ERROR, &str, &str)] = codes![
    (ERROR_SUCCESS, "The function succeeded"),
    (
        ERROR_INVALID_PARAMETER,
        "The combination of parameters and flags that are specified is invalid."
    ),
    (
        ERROR_NOT_SUPPORTED,
        "The system is not running a graphics driver that was written according to the Windows Display Driver Model (WDDM). The function is only supported on a system with a WDDM driver running."
    ),
    (
        ERROR_ACCESS_DENIED,
        "The caller does not have access to the console session. This error occurs if the calling process does not have access to the current desktop or is running on a remote session."
    ),
    (ERROR_GEN_FAILURE, "An unspecified error occurred."),
    (
        ERROR_INSUFFICIENT_BUFFER,
        "The supplied path and mode buffer are too small."
    ),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Win32Error(WIN32_ERROR);

impl Win32Error {
    pub(crate) fn is_ok(self) -> bool {
        self.0.is_ok()
    }

    #[cfg(feature = "dump")]
    pub(crate) fn is_err(self) -> bool {
        self.0.is_err()
    }

    pub fn to_result<T>(self, function: &'static str, value: T) -> Result<T> {
        self.to_result_with(function, || value)
    }

    pub fn to_result_with<T>(self, function: &'static str, f: impl FnOnce() -> T) -> Result<T> {
        if self.is_ok() {
            Ok(f())
        } else {
            Err(Error::WindowsApiError {
                function,
                code: self.0,
                message: self.message(),
            })
        }
    }

    pub(crate) fn message(self) -> String {
        if let Some((name, text)) = self.code_name_and_text() {
            format!("{name} - {text}")
        } else {
            format!("{:?} - Unknown error", self.0.0)
        }
    }

    fn code_name_and_text(self) -> Option<(&'static str, &'static str)> {
        CODES
            .iter()
            .find_map(|(code, name, text)| (self.0 == *code).then_some((*name, *text)))
    }
}

impl From<WIN32_ERROR> for Win32Error {
    fn from(error: WIN32_ERROR) -> Self {
        Win32Error(error)
    }
}

impl From<u32> for Win32Error {
    fn from(value: u32) -> Self {
        Self::from(WIN32_ERROR(value))
    }
}

impl From<i32> for Win32Error {
    fn from(value: i32) -> Self {
        Self::from(value.cast_unsigned())
    }
}
