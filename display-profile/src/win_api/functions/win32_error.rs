use std::hash::Hash;

use anyhow::{Result, bail};
use windows::Win32::Foundation as WinFoundation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Win32Error(pub WinFoundation::WIN32_ERROR);

impl Win32Error {
    pub fn into_unit_result(self) -> Result<()> {
        self.into_result(())
    }

    pub fn into_result<T>(self, value: T) -> Result<T> {
        self.into_result_with(|| value)
    }

    pub fn into_result_with<T>(self, f: impl FnOnce() -> T) -> Result<T> {
        if self.0.is_ok() {
            Ok(f())
        } else {
            bail!(self.message())
        }
    }

    fn message(self) -> String {
        match self.0 {
            WinFoundation::ERROR_SUCCESS => "The operation completed successfully.".to_string(),
            WinFoundation::ERROR_INVALID_PARAMETER => "The combination of parameters and flags specified is invalid.".to_string(),
            WinFoundation::ERROR_NOT_SUPPORTED => "The system is not running a graphics driver that was written according to the Windows Display Driver Model (WDDM). The function is only supported on a system with a WDDM driver running.".to_string(),
            WinFoundation::ERROR_ACCESS_DENIED => "The caller does not have access to the console session. This error occurs if the calling process does not have access to the current desktop or is running on a remote session.".to_string(),
            WinFoundation::ERROR_GEN_FAILURE => "An unspecified error occurred.".to_string(),
            WinFoundation::ERROR_BAD_CONFIGURATION => "The function could not find a workable solution for the source and target modes that the caller did not specify.".to_string(),
            win32_error => format!("Win32 error code: {}", win32_error.0),
        }
    }
}

impl From<WinFoundation::WIN32_ERROR> for Win32Error {
    fn from(value: WinFoundation::WIN32_ERROR) -> Self {
        Self(value)
    }
}

impl From<i32> for Win32Error {
    fn from(value: i32) -> Self {
        Self(WinFoundation::WIN32_ERROR(value.cast_unsigned()))
    }
}

impl Hash for Win32Error {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.0.hash(state);
    }
}
