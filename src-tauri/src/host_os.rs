#[cfg(not(windows))]
mod linux;
#[cfg(not(windows))]
pub use linux::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::*;

use anyhow::Result;

use crate::options_types::Display;

pub trait CurrentDisplayHandler {
    fn get(&self) -> Display;
    fn switch_to(&self, display: Display) -> Result<()>;
}
