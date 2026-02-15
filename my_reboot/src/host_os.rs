#[cfg(not(windows))]
mod linux;
#[cfg(not(windows))]
pub use linux::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::*;

use std::process::ExitStatus;

use anyhow::{Result, bail};

use crate::script::Script;

pub struct PredefinedScript {
    pub button_label: &'static str,
    pub script: Script,
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
