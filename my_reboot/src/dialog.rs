use anyhow::Result;

use crate::script::Script;
use crate::state::StateProvider;

mod advanced;

pub fn show_advanced() -> Result<()> {
    let provider = StateProvider::new()?;
    let state = provider.get_state();
    let options = advanced::Options {
        next_boot_operating_system: state.next_boot_operating_system,
    };

    let outcome = advanced::show(options)?;

    if let Some(options) = outcome {
        let script = Script {
            next_boot_operating_system: Some(options.next_boot_operating_system.into()),
            next_windows_boot_display: None,
            switch_to_display: None,
            reboot_action: None,
        };
        script.execute()?
    }

    Ok(())
}
