mod args;
mod dialog;
mod host_os;
mod options_types;
mod script;
mod state;
mod text;

mod persist {
    pub(crate) mod configs;
    pub(crate) mod grubenv;
    pub(crate) mod options;
}

use anyhow::{Context, Result};
use dialog::Mode;
use host_os::PREDEFINED_SCRIPTS;
use script::Script;
#[cfg(windows)]
use script::SwitchToProfile;

use crate::args::ParsedArgs;
use crate::options_types::{LabeledProfile, OptionType, ProfileId};
use crate::persist::configs::Configs;
use crate::state::StateProvider;

fn main() -> Result<()> {
    let args = args::parse()
        .with_context(|| "Argumentos inválidos.\nPara ajuda, execute: my-reboot --help")?;

    match args {
        ParsedArgs::Dialog(mode) => show_dialog(mode),
        ParsedArgs::Script(script) => execute_script(script),
        ParsedArgs::ShowState => show_state(),
        ParsedArgs::Configure => configure(),
        ParsedArgs::Usage => {
            show_usage();
            Ok(())
        }
    }
}

fn show_dialog(mode: Mode) -> Result<()> {
    #[cfg(windows)]
    {
        // Hide the console window.
        use windows::Win32::System::Console::GetConsoleWindow;
        use windows::Win32::UI::WindowsAndMessaging::{SW_HIDE, ShowWindow};
        unsafe {
            let window = GetConsoleWindow();
            if !window.is_invalid() {
                let _ = ShowWindow(window, SW_HIDE);
            }
        }
    }

    let provider = StateProvider::new()?;

    let labels: Vec<_> = PREDEFINED_SCRIPTS
        .iter()
        .map(|ps| ps.resolve_label(provider.configs()))
        .collect();

    let state = provider.state()?;
    let script_options = dialog::ScriptOptions {
        next_boot_operating_system: state.next_boot_operating_system,
        next_windows_boot_profile: state.next_windows_boot_profile,
        #[cfg(windows)]
        switch_profile: false,
        reboot_action: None,
    };
    let profile_labels =
        ProfileId::values().map(|id| LabeledProfile::get(id, provider.configs()).to_string());

    let outcome = dialog::show(mode, labels, script_options, profile_labels)?;

    match outcome {
        Some(dialog::Outcome::PredefinedScriptIndex(index)) => {
            PREDEFINED_SCRIPTS[index].script.execute()
        }
        Some(dialog::Outcome::ScriptOptions(options)) => {
            let script = Script {
                next_boot_operating_system: Some(options.next_boot_operating_system.into()),
                next_windows_boot_profile: Some(options.next_windows_boot_profile.into()),
                #[cfg(windows)]
                switch_to_profile: options.switch_profile.then_some(SwitchToProfile::Other),
                reboot_action: options.reboot_action,
            };
            script.execute()
        }
        None => Ok(()),
    }
}

fn execute_script(script: Script) -> Result<()> {
    script.execute()
}

fn show_state() -> Result<()> {
    let provider = StateProvider::new()?;
    let state = provider.state()?;

    println!(
        "{}: {}",
        text::operating_system::ON_NEXT_BOOT_DESCRIPTION,
        text::operating_system::value_text(state.next_boot_operating_system)
    );
    println!(
        "{}: {}",
        text::profile::ON_NEXT_WINDOWS_BOOT_DESCRIPTION,
        text::profile::next_boot_value_text(
            state
                .next_windows_boot_profile
                .map(|id| LabeledProfile::get(id, provider.configs()))
        )
    );
    #[cfg(windows)]
    println!(
        "{}: {}",
        text::profile::CURRENT,
        text::profile::current_value_text(
            state
                .current_profile
                .map(|id| LabeledProfile::get(id, provider.configs()))
        )
    );

    Ok(())
}

fn configure() -> Result<()> {
    host_os::configuration::configure()
}

fn show_usage() {
    let profile_labels =
        Configs::load().map(|configs| [configs.profile.a.label, configs.profile.b.label]);

    let usage = args::Usage::new(profile_labels);

    println!("{usage}");
}
