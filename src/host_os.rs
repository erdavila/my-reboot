use std::path::PathBuf;
use std::process::ExitStatus;

use anyhow::{Result, bail};
#[cfg(not(windows))]
pub use linux::*;
#[cfg(windows)]
pub use windows::*;

use crate::script::SetOrUnset;

#[cfg(not(windows))]
mod linux;
#[cfg(windows)]
mod windows;

pub(crate) fn state_path(filename: &str) -> PathBuf {
    let state_dir_path = option_env!("STATE_DIR_PATH").unwrap_or(DEFAULT_STATE_DIR_PATH);
    PathBuf::from(state_dir_path).join(filename)
}

pub(crate) struct TemplateResolver {
    label: String,
}
impl TemplateResolver {
    pub(crate) fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
        }
    }

    pub(crate) fn resolve_set_or_unset_option<T: ToString + Copy>(
        &mut self,
        pattern: &str,
        option: Option<SetOrUnset<T>>,
        undefined_text: &str,
    ) {
        let option = Self::set_or_unset_option_to_option(option);
        self.resolve_option(pattern, option, undefined_text);
    }

    pub(crate) fn resolve_set_or_unset_option_with<T: Copy>(
        &mut self,
        pattern: &str,
        option: Option<SetOrUnset<T>>,
        f: impl FnOnce(T) -> String,
        undefined_text: &str,
    ) {
        let option = Self::set_or_unset_option_to_option(option);
        self.resolve_option_with(pattern, option, f, undefined_text);
    }

    pub(crate) fn resolve_option<T: ToString>(
        &mut self,
        pattern: &str,
        option: Option<T>,
        undefined_text: &str,
    ) {
        self.resolve_option_with(pattern, option, |op| op.to_string(), undefined_text);
    }

    pub(crate) fn resolve_option_with<T>(
        &mut self,
        pattern: &str,
        option: Option<T>,
        f: impl FnOnce(T) -> String,
        undefined_text: &str,
    ) {
        let replacement = option.map_or_else(|| format!("[{undefined_text}]"), f);
        self.label = self.label.replace(&format!("{{{pattern}}}"), &replacement);
    }

    fn set_or_unset_option_to_option<T: Copy>(option: Option<SetOrUnset<T>>) -> Option<T> {
        option.and_then(SetOrUnset::into_option)
    }

    pub(crate) fn into_label(self) -> String {
        self.label
    }
}

pub trait SuccessOr {
    fn success_or(self, message: &'static str) -> Result<()>;
}

impl SuccessOr for ExitStatus {
    fn success_or(self, message: &'static str) -> Result<()> {
        if self.success() {
            Ok(())
        } else {
            bail!(message)
        }
    }
}
