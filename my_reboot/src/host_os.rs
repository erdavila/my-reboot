use std::process::ExitStatus;

use anyhow::{Result, bail};
#[cfg(not(windows))]
pub use linux::*;
#[cfg(windows)]
pub use windows::*;

use crate::configs::Configs;
use crate::options_types::LabeledProfile;
use crate::script::{Script, SetOrUnset};
use crate::text::{self, Capitalized};

#[cfg(not(windows))]
mod linux;
#[cfg(windows)]
mod windows;

pub struct PredefinedScript {
    pub button_label_template: &'static str,
    pub script: Script,
}
impl PredefinedScript {
    pub(crate) fn resolve_label(&self, configs: &Configs) -> String {
        let profile_label = |profile_id| LabeledProfile::get(profile_id, configs).to_string();

        let mut template_resolver = TemplateResolver::new(self.button_label_template);

        template_resolver.resolve_set_or_unset_option(
            "next_boot_operating_system",
            self.script.next_boot_operating_system,
            text::operating_system::UNDEFINED,
        );
        template_resolver.resolve_set_or_unset_option_with(
            "next_windows_boot_profile",
            self.script.next_windows_boot_profile,
            profile_label,
            text::profile::UNDEFINED,
        );
        #[cfg(windows)]
        template_resolver.resolve_option_with(
            "switch_to_profile",
            self.script.switch_to_profile,
            |switch_to| {
                use crate::script::SwitchToProfile;
                match switch_to {
                    SwitchToProfile::Other => "outro".to_string(),
                    SwitchToProfile::Profile(profile_id) => profile_label(profile_id),
                    SwitchToProfile::Saved => "salvo".to_string(),
                }
            },
            text::profile::UNDEFINED,
        );
        template_resolver.resolve_option(
            "reboot_action",
            self.script.reboot_action,
            text::reboot_action::UNDEFINED,
        );

        Capitalized(template_resolver.label).to_string()
    }
}

struct TemplateResolver {
    label: String,
}
impl TemplateResolver {
    fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
        }
    }

    fn resolve_set_or_unset_option<T: ToString + Copy>(
        &mut self,
        pattern: &str,
        option: Option<SetOrUnset<T>>,
        undefined_text: &str,
    ) {
        let option = Self::set_or_unset_option_to_option(option);
        self.resolve_option(pattern, option, undefined_text);
    }

    fn resolve_set_or_unset_option_with<T: Copy>(
        &mut self,
        pattern: &str,
        option: Option<SetOrUnset<T>>,
        f: impl FnOnce(T) -> String,
        undefined_text: &str,
    ) {
        let option = Self::set_or_unset_option_to_option(option);
        self.resolve_option_with(pattern, option, f, undefined_text);
    }

    fn resolve_option<T: ToString>(
        &mut self,
        pattern: &str,
        option: Option<T>,
        undefined_text: &str,
    ) {
        self.resolve_option_with(pattern, option, |op| op.to_string(), undefined_text);
    }

    fn resolve_option_with<T>(
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
        option.and_then(SetOrUnset::to_option)
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
