use std::env;

use ansi_term::{ANSIString, Color};
use anyhow::{Ok, Result, anyhow};

use crate::options_types::{Display, OperatingSystem, OptionType, RebootAction};
use crate::state::StateProvider;
use crate::{host_os, text};

#[derive(Clone, Copy, Debug)]
pub struct Script {
    pub next_boot_operating_system: Option<SetOrUnset<OperatingSystem>>,
    pub next_windows_boot_display: Option<SetOrUnset<Display>>,
    pub switch_to_display: Option<SwitchToDisplay>,
    pub reboot_action: Option<RebootAction>,
}
impl Script {
    pub const fn new() -> Self {
        Script {
            next_boot_operating_system: None,
            next_windows_boot_display: None,
            switch_to_display: None,
            reboot_action: None,
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

        if let Some(switch_to) = &script.switch_to_display {
            self.apply_switch_to_display(switch_to)?;
        }

        if let Some(reboot_action) = &script.reboot_action {
            self.apply_reboot_action(reboot_action)?;
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

    fn apply_switch_to_display(&mut self, switch_to: &SwitchToDisplay) -> Result<()> {
        let from_display = self
            .state_provider
            .get_current_display()
            .ok_or_else(|| anyhow!(text::display::switching::NOT_SUPPORTED))?;

        match switch_to {
            SwitchToDisplay::Other => {
                let to_display: Vec<_> = Display::values()
                    .into_iter()
                    .filter(|d| *d != from_display)
                    .collect();
                assert_eq!(to_display.len(), 1);
                let to_display = to_display[0];

                self.switch_display_to(to_display)?;
            }
            SwitchToDisplay::Display(to_display) => {
                if *to_display == from_display {
                    println!(
                        "{} {}",
                        text::display::value_text(Some(*to_display)),
                        text::display::switching::IS_ALREADY_CURRENT
                    );
                } else {
                    self.switch_display_to(*to_display)?;
                }
            }
            SwitchToDisplay::Saved => match self.state_provider.get_next_windows_boot_display() {
                Some(to_display) => {
                    if to_display == from_display {
                        println!(
                            "A {} é {}, que já é a tela atual",
                            text::display::ON_NEXT_WINDOWS_BOOT_DESCRIPTION,
                            text::display::value_text(Some(to_display))
                        )
                    } else {
                        self.switch_display_to(to_display)?;
                        self.state_provider.unset_next_windows_boot_display();
                    }
                }
                None => println!(
                    "A {} é {}",
                    text::display::ON_NEXT_WINDOWS_BOOT_DESCRIPTION,
                    text::display::value_text(None)
                ),
            },
        }

        Ok(())
    }

    fn switch_display_to(&self, display: Display) -> Result<()> {
        println!(
            "{} {}",
            text::display::switching::TO,
            text::display::value_text(Some(display))
        );
        self.state_provider.set_current_display(display)
    }

    fn apply_reboot_action(&self, reboot_action: &RebootAction) -> Result<()> {
        match reboot_action {
            RebootAction::Reboot => self.do_reboot_action(host_os::reboot, "Reiniciando"),
            RebootAction::Shutdown => self.do_reboot_action(host_os::shutdown, "Desligando"),
        }
    }

    fn do_reboot_action(&self, method: fn() -> Result<()>, message: &str) -> Result<()> {
        println!("{}...", message);
        if env::var("NO_REBOOT_ACTION").is_ok() {
            println!("{} 😬", Color::Yellow.paint("...mas não de verdade!"));
            Ok(())
        } else {
            method()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SetOrUnset<T> {
    Set(T),
    Unset,
}
impl<T: Copy> SetOrUnset<T> {
    fn to_option(self) -> Option<T> {
        match self {
            SetOrUnset::Set(value) => Some(value),
            SetOrUnset::Unset => None,
        }
    }
}
impl<T> From<Option<T>> for SetOrUnset<T> {
    fn from(option: Option<T>) -> Self {
        match option {
            Some(value) => Self::Set(value),
            None => Self::Unset,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SwitchToDisplay {
    Other,
    Display(Display),
    Saved,
}
