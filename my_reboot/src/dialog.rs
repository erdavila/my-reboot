use anyhow::Result;

use crate::script::Script;
#[cfg(windows)]
use crate::script::SwitchToDisplay;
use crate::state::StateProvider;

mod advanced;

pub fn show_advanced() -> Result<()> {
    let provider = StateProvider::new()?;
    let state = provider.get_state();
    let options = advanced::Options {
        next_boot_operating_system: state.next_boot_operating_system,
        next_windows_boot_display: state.next_windows_boot_display,
        #[cfg(windows)]
        switch_display: false,
        reboot_action: None,
    };

    let outcome = advanced::show(options)?;

    if let Some(options) = outcome {
        let script = Script {
            next_boot_operating_system: Some(options.next_boot_operating_system.into()),
            next_windows_boot_display: Some(options.next_windows_boot_display.into()),
            #[cfg(windows)]
            switch_to_display: options.switch_display.then_some(SwitchToDisplay::Other),
            #[cfg(not(windows))]
            switch_to_display: None,
            reboot_action: options.reboot_action,
        };
        script.execute()?
    }

    Ok(())
}
