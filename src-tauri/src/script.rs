use ansi_term::ANSIString;
use anyhow::{Ok, Result};

use crate::options_types::{Display, OperatingSystem, OptionType};
use crate::state::StateProvider;
use crate::text;

pub struct Script {
    pub next_boot_operating_system: Option<SetOrUnset<OperatingSystem>>,
    pub next_windows_boot_display: Option<SetOrUnset<Display>>,
}
impl Script {
    pub fn new() -> Self {
        Script {
            next_boot_operating_system: None,
            next_windows_boot_display: None,
        }
    }

    pub fn execute(&self) -> Result<()> {
        let mut executor = ScriptExecutor {
            state_provider: StateProvider::new()?,
        };

        executor.execute(self)
    }
}

struct ScriptExecutor {
    state_provider: StateProvider,
}
impl ScriptExecutor {
    fn execute(&mut self, script: &Script) -> Result<()> {
        if let Some(os_option) = &script.next_boot_operating_system {
            self.apply_next_boot_operating_system(os_option);
        }

        if let Some(display_option) = &script.next_windows_boot_display {
            self.apply_next_windows_boot_display(display_option);
        }

        Ok(())
    }

    fn apply_next_boot_operating_system(&mut self, os_option: &SetOrUnset<OperatingSystem>) {
        self.apply_option(
            os_option,
            StateProvider::set_next_boot_operating_system,
            StateProvider::unset_next_boot_operating_system,
            text::operating_system::ON_NEXT_BOOT_DESCRIPTION,
            text::operating_system::WAS_UPDATED_TO,
            text::operating_system::value_text,
        );
    }

    fn apply_next_windows_boot_display(&mut self, display_option: &SetOrUnset<Display>) {
        self.apply_option(
            display_option,
            StateProvider::set_next_windows_boot_display,
            StateProvider::unset_next_windows_boot_display,
            text::display::ON_NEXT_WINDOWS_BOOT_DESCRIPTION,
            text::display::WAS_UPDATED_TO,
            text::display::value_text,
        );
    }

    fn apply_option<T, S, U, V>(
        &mut self,
        option: &SetOrUnset<T>,
        set: S,
        unset: U,
        description: &str,
        was_updated_to: &str,
        value_text: V,
    ) where
        T: OptionType,
        S: FnOnce(&mut StateProvider, T),
        U: FnOnce(&mut StateProvider),
        V: Fn(Option<T>) -> ANSIString<'static>,
    {
        use SetOrUnset::*;

        match option {
            Set(value) => set(&mut self.state_provider, *value),
            Unset => unset(&mut self.state_provider),
        }

        println!(
            "{} {} {}.",
            description,
            was_updated_to,
            value_text(option.to_option())
        );
    }
}

#[derive(PartialEq, Eq, Debug)]
pub enum SetOrUnset<T> {
    Set(T),
    Unset,
}
impl<T: Copy> SetOrUnset<T> {
    fn to_option(&self) -> Option<T> {
        match self {
            SetOrUnset::Set(value) => Some(*value),
            SetOrUnset::Unset => None,
        }
    }
}
