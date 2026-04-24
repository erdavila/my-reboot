use windows::Win32::Foundation::{
    ERROR_ACCESS_DENIED, ERROR_GEN_FAILURE, ERROR_INSUFFICIENT_BUFFER, ERROR_INVALID_PARAMETER,
    ERROR_NOT_SUPPORTED, ERROR_SUCCESS, WIN32_ERROR,
};

use crate::Result;
use crate::error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Win32Error(WIN32_ERROR);

impl Win32Error {
    fn is_ok(self) -> bool {
        self.0.is_ok()
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

    fn message(self) -> String {
        match self.0 {
            ERROR_SUCCESS => "The function succeeded".to_string(),
            ERROR_INVALID_PARAMETER => "The combination of parameters and flags that are specified is invalid.".to_string(),
            ERROR_NOT_SUPPORTED => "The system is not running a graphics driver that was written according to the Windows Display Driver Model (WDDM). The function is only supported on a system with a WDDM driver running.".to_string(),
            ERROR_ACCESS_DENIED => "The caller does not have access to the console session. This error occurs if the calling process does not have access to the current desktop or is running on a remote session.".to_string(),
            ERROR_GEN_FAILURE => "An unspecified error occurred.".to_string(),
            ERROR_INSUFFICIENT_BUFFER => "The supplied path and mode buffer are too small.".to_string(),
            _ => format!("Unknown error {:?}", self.0),
        }
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
