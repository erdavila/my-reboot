mod args;
mod configuration;
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

use std::num::NonZeroUsize;

use anyhow::{Context, Result, bail};
use dialog::Mode;
use script::Script;
#[cfg(all(windows, not(test)))]
use script::SwitchToProfile;

use crate::args::{ParsedArgs, PredefinedScriptParsedArgs};
use crate::host_os::HOST_OS;
use crate::options_types::{LabeledProfile, ProfileId, SerializeToString, Values as _};
use crate::persist::configs::Configs;
use crate::state::StateProvider;

fn main() -> Result<()> {
    let args = args::parse()
        .with_context(|| "Argumentos inválidos.\nPara ajuda, execute: my-reboot --help")?;

    match args {
        ParsedArgs::Dialog(mode) => show_dialog(mode),
        ParsedArgs::Script(script) => execute_script(script),
        ParsedArgs::PredefinedScript(PredefinedScriptParsedArgs::Number(number)) => {
            execute_predefined_script(number)
        }
        ParsedArgs::PredefinedScript(PredefinedScriptParsedArgs::List) => list_predefined_scripts(),
        ParsedArgs::ShowState => show_state(),
        ParsedArgs::Configure => configure(),
        ParsedArgs::Usage => {
            show_usage();
            Ok(())
        }
        ParsedArgs::Version => {
            show_version();
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

    let labels: Vec<_> = provider.configs().operating_system[HOST_OS]
        .scripts
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
            provider.configs().operating_system[HOST_OS].scripts[index]
                .script
                .execute()
        }
        Some(dialog::Outcome::ScriptOptions(options)) => {
            let script = Script {
                next_boot_operating_system: Some(options.next_boot_operating_system.into()),
                next_windows_boot_profile: Some(options.next_windows_boot_profile.into()),
                switch_to_profile: cfg_select! {
                    all(windows, not(test)) => options.switch_profile.then_some(SwitchToProfile::Other),
                    _ => None,
                },
                reboot_action: options.reboot_action,
            };
            script.execute()
        }
        None => Ok(()),
    }
}

fn execute_predefined_script(number: NonZeroUsize) -> Result<()> {
    let index = number.get() - 1;

    let configs = Configs::load()?;
    let predef_scripts = &configs.operating_system[HOST_OS].scripts;
    let Some(predef_script) = predef_scripts.get(index) else {
        bail!(
            "Número inválido de script para o sistema operacional atual (mín: 1; máx: {})",
            predef_scripts.len()
        );
    };

    println!(
        "Executando script '{}'",
        predef_script.resolve_label(&configs)
    );
    predef_script.script.execute()
}

fn list_predefined_scripts() -> Result<()> {
    let configs = Configs::load()?;

    for (i, predef_script) in configs.operating_system[HOST_OS].scripts.iter().enumerate() {
        let number = i + 1;

        let label = predef_script.resolve_label(&configs);
        let Script {
            next_boot_operating_system,
            next_windows_boot_profile,
            switch_to_profile,
            reboot_action,
        } = &predef_script.script;

        macro_rules! print_option {
            ($name:ident) => {
                $name.inspect(|value| {
                    println!("  {}: {}", stringify!($name), value.serialize_to_string())
                });
            };
        }

        println!("{number}: '{label}'");
        print_option!(next_boot_operating_system);
        print_option!(next_windows_boot_profile);
        print_option!(switch_to_profile);
        print_option!(reboot_action);
        println!();
    }

    Ok(())
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
    configuration::configure()
}

fn show_usage() {
    let profile_labels =
        Configs::load().map(|configs| [configs.profile.a.label, configs.profile.b.label]);

    let usage = args::Usage::new(profile_labels);

    println!("{usage}");
}

fn show_version() {
    println!(
        "{} {} ({})",
        env!("MY_REBOOT_NAME"),
        env!("MY_REBOOT_VERSION"),
        env!("MY_REBOOT_TIMESTAMP")
    );

    let vcs_revision = env!("MY_REBOOT_VCS_REVISION");
    if let Some(jj_ids) = vcs_revision.strip_prefix("JJ:") {
        let (change_id, commit_id) = jj_ids.split_once(' ').unwrap();
        println!("JJ change/commit ID: {change_id}/{commit_id}");
    } else {
        let git_head = vcs_revision.strip_prefix("Git:").unwrap();
        println!("Git HEAD: {git_head}");
    }
}
