use serde::Deserialize;

use super::errors::{self, ArgError};
use crate::options_types::{DeserializeFromString as _, OperatingSystem, RebootAction};
#[cfg(any(windows, test))]
use crate::script::SwitchToProfile;
use crate::script::{Script, SetOrUnset};
use crate::text;

pub(super) const NEXT_BOOT_OPERATING_SYSTEM_PREFIX: &str = "os";
pub(super) const NEXT_WINDOWS_BOOT_PROFILE_PREFIX: &str = "profile";
#[cfg(any(windows, test))]
pub(super) const SWITCH_TO_PROFILE_PREFIX: &str = "switch";

pub fn parse(
    arg: &str,
    args: &mut impl Iterator<Item = String>,
) -> Result<Option<Script>, ArgError> {
    let mut script = Script::new();

    if !parse_single(arg, &mut script)? {
        return Ok(None);
    }

    for arg in args {
        if !parse_single(&arg, &mut script)? {
            return errors::unknown_argument_error(&arg);
        }
    }

    Ok(Some(script))
}

fn parse_single(arg: &str, script: &mut Script) -> Result<bool, ArgError> {
    if parse_next_boot_operating_system(arg, script)? {
        return Ok(true);
    }

    if parse_next_windows_boot_profile(arg, script)? {
        return Ok(true);
    }

    #[cfg(any(windows, test))]
    if parse_switch_to_profile(arg, script)? {
        return Ok(true);
    }

    if parse_reboot_action(arg, script)? {
        return Ok(true);
    }

    Ok(false)
}

fn parse_next_boot_operating_system(arg: &str, script: &mut Script) -> Result<bool, ArgError> {
    let with_prefix = || SetOrUnset::from_str_with_prefix(arg, NEXT_BOOT_OPERATING_SYSTEM_PREFIX);
    let without_prefix = || OperatingSystem::deserialize_from_string(arg).map(SetOrUnset::Set);

    set_option(
        with_prefix().or_else(without_prefix),
        &mut script.next_boot_operating_system,
        text::operating_system::ON_NEXT_BOOT_DESCRIPTION,
        arg,
    )
}

fn parse_next_windows_boot_profile(arg: &str, script: &mut Script) -> Result<bool, ArgError> {
    set_option(
        SetOrUnset::from_str_with_prefix(arg, NEXT_WINDOWS_BOOT_PROFILE_PREFIX),
        &mut script.next_windows_boot_profile,
        text::profile::ON_NEXT_WINDOWS_BOOT_DESCRIPTION,
        arg,
    )
}

#[cfg(any(windows, test))]
fn parse_switch_to_profile(arg: &str, script: &mut Script) -> Result<bool, ArgError> {
    let full = || {
        strip_prefix(arg, SWITCH_TO_PROFILE_PREFIX)
            .and_then(SwitchToProfile::deserialize_from_string)
    };
    let prefix_only = || (arg == SWITCH_TO_PROFILE_PREFIX).then_some(SwitchToProfile::Other);

    set_option(
        full().or_else(prefix_only),
        &mut script.switch_to_profile,
        text::profile::SWITCH_DESCRIPTION,
        arg,
    )
}

fn parse_reboot_action(arg: &str, script: &mut Script) -> Result<bool, ArgError> {
    set_option(
        RebootAction::deserialize_from_string(arg),
        &mut script.reboot_action,
        text::reboot_action::ACTION_DESCRIPTION,
        arg,
    )
}

fn set_option<T>(
    option: Option<T>,
    value: &mut Option<T>,
    descr: &str,
    arg: &str,
) -> Result<bool, ArgError> {
    if let Some(option) = option {
        if value.is_none() {
            value.replace(option);
            Ok(true)
        } else {
            Err(ArgError::new(
                &format!("A opção de {descr} não pode ser usada mais de uma vez"),
                arg,
            ))
        }
    } else {
        Ok(false)
    }
}

impl<'de, T: Deserialize<'de>> SetOrUnset<T> {
    pub(crate) fn from_str_with_prefix(s: &str, prefix: &str) -> Option<Self> {
        strip_prefix(s, prefix).and_then(SetOrUnset::deserialize_from_string)
    }
}

fn strip_prefix<'a>(arg: &'a str, prefix: &str) -> Option<&'a str> {
    (arg.starts_with(prefix) && arg.chars().nth(prefix.len()) == Some(':'))
        .then(|| &arg[prefix.len() + 1..])
}

#[cfg(test)]
mod tests {
    use std::iter;

    use SetOrUnset::*;

    use super::*;
    use crate::options_types::{OperatingSystem, ProfileId, RebootAction};

    #[test]
    fn test_parse() {
        let arg = "os:windows";
        let mut args = iter::empty();

        let result = parse(arg, &mut args);

        let option = result.expect("result should be Ok(_)");
        let script = option.expect("option should be Some(_)");
        assert_eq!(
            script.next_boot_operating_system,
            Some(Set(OperatingSystem::Windows))
        );
    }

    #[test]
    fn test_parse_unrecognized_arg() {
        let arg = "blah";
        let mut args = iter::empty();

        let result = parse(arg, &mut args);

        let option = result.expect("result should be Ok(_)");
        assert!(option.is_none());
    }

    #[test]
    fn test_parse_multiple_args() {
        let arg = "profile:a";
        let mut args = ["os:windows".to_string()].into_iter();

        let result = parse(arg, &mut args);

        let option = result.expect("result should be Ok(_)");
        let script = option.expect("option should be Some(_)");
        assert_eq!(
            script.next_boot_operating_system,
            Some(Set(OperatingSystem::Windows))
        );
        assert_eq!(script.next_windows_boot_profile, Some(Set(ProfileId::A)));
    }

    #[test]
    fn test_parse_multiple_args_invalid() {
        let arg = "os:windows";
        let mut args = ["blah".to_string()].into_iter();

        let result = parse(arg, &mut args);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_single_os() {
        let mut script = Script::new();

        let result = parse_single("os:windows", &mut script);

        let success = result.expect("result should be Ok(_)");
        assert!(success);
        assert_eq!(
            script.next_boot_operating_system,
            Some(Set(OperatingSystem::Windows))
        );
    }

    #[test]
    fn test_parse_single_profile() {
        let mut script = Script::new();

        let result = parse_single("profile:a", &mut script);

        let success = result.expect("result should be Ok(_)");
        assert!(success);
        assert_eq!(script.next_windows_boot_profile, Some(Set(ProfileId::A)));
    }

    #[test]
    fn test_parse_single_switch_to_profile() {
        let mut script = Script::new();

        let result = parse_single("switch:a", &mut script);

        let success = result.expect("result should be Ok(_)");
        assert!(success);
        assert_eq!(
            script.switch_to_profile,
            Some(SwitchToProfile::Profile(ProfileId::A))
        );
    }

    #[test]
    fn test_parse_single_reboot_action() {
        let mut script = Script::new();

        let result = parse_single("reboot", &mut script);

        let success = result.expect("result should be Ok(_)");
        assert!(success);
        assert_eq!(script.reboot_action, Some(RebootAction::Reboot));
    }

    #[test]
    fn test_parse_single_no_script_arg() {
        let mut script = Script::new();

        let result = parse_single("blah", &mut script);

        let success = result.expect("result should be Ok(_)");
        assert!(!success);
    }

    #[test]
    fn test_parse_single_invalid() {
        let mut script = Script::new();

        let result = parse_single("profile:blah", &mut script);

        assert_eq!(result, Ok(false));
        assert_eq!(script, Script::new());
    }

    #[test]
    fn test_parse_next_boot_operating_system() {
        let cases = [
            ("os:windows", Set(OperatingSystem::Windows)),
            ("windows", Set(OperatingSystem::Windows)),
            ("os:linux", Set(OperatingSystem::Linux)),
            ("linux", Set(OperatingSystem::Linux)),
            ("os:unset", Unset),
        ];

        for (arg, expected) in cases {
            let mut script = Script::new();

            let result = parse_next_boot_operating_system(arg, &mut script);

            assert_eq!(result, Ok(true), "Result for argument \"{arg}\"");
            assert_eq!(script.next_boot_operating_system, Some(expected));
        }
    }

    #[test]
    fn test_parse_next_boot_operating_system_invalid() {
        let mut script = Script::new();

        let result = parse_next_boot_operating_system("os:blah", &mut script);

        assert_eq!(result, Ok(false));
        assert_eq!(script.next_boot_operating_system, None);
    }

    #[test]
    fn test_parse_next_boot_operating_system_not_os_arg() {
        let mut script = Script::new();
        let arg = "blah".to_string();

        let result = parse_next_boot_operating_system(&arg, &mut script);

        assert_eq!(result, Ok(false), "Result for argument \"{arg}\"");
    }

    #[test]
    fn test_parse_next_boot_operating_system_already_set() {
        let mut script = Script::new();
        script.next_boot_operating_system = Some(Unset);

        let result = parse_next_boot_operating_system("os:windows", &mut script);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_next_windows_boot_profile() {
        let cases = [
            ("profile:a", Set(ProfileId::A)),
            ("profile:b", Set(ProfileId::B)),
            ("profile:unset", Unset),
        ];

        for (arg, expected) in cases {
            let mut script = Script::new();

            let result = parse_next_windows_boot_profile(arg, &mut script);

            assert_eq!(result, Ok(true), "Result for argument \"{arg}\"");
            assert_eq!(script.next_windows_boot_profile, Some(expected));
        }
    }

    #[test]
    fn test_parse_next_windows_boot_profile_invalid() {
        let mut script = Script::new();

        let result = parse_next_windows_boot_profile("profile:blah", &mut script);

        assert_eq!(result, Ok(false));
        assert_eq!(script.next_windows_boot_profile, None);
    }

    #[test]
    fn test_parse_next_windows_boot_profile_not_profile_arg() {
        let mut script = Script::new();
        let arg = "blah".to_string();

        let result = parse_next_windows_boot_profile(&arg, &mut script);

        assert_eq!(result, Ok(false), "Result for argument \"{arg}\"");
    }

    #[test]
    fn test_parse_next_windows_boot_profile_already_set() {
        let mut script = Script::new();
        script.next_windows_boot_profile = Some(Unset);

        let result = parse_next_windows_boot_profile("profile:a", &mut script);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_switch_to_profile() {
        let cases = [
            ("switch", SwitchToProfile::Other),
            ("switch:other", SwitchToProfile::Other),
            ("switch:a", SwitchToProfile::Profile(ProfileId::A)),
            ("switch:b", SwitchToProfile::Profile(ProfileId::B)),
            ("switch:saved", SwitchToProfile::Saved),
        ];

        for (arg, expected) in cases {
            let mut script = Script::new();

            let result = parse_switch_to_profile(arg, &mut script);

            assert_eq!(result, Ok(true), "Result for argument \"{arg}\"");
            assert_eq!(script.switch_to_profile, Some(expected));
        }
    }

    #[test]
    fn test_parse_switch_to_profile_invalid() {
        let mut script = Script::new();

        let result = parse_switch_to_profile("switch:blah", &mut script);

        assert_eq!(result, Ok(false));
        assert_eq!(script.switch_to_profile, None);
    }

    #[test]
    fn test_parse_switch_to_profile_no_switch_arg() {
        let mut script = Script::new();
        let arg = "blah";

        let result = parse_switch_to_profile(arg, &mut script);

        assert_eq!(result, Ok(false), "Result for argument \"{arg}\"");
    }

    #[test]
    fn test_parse_switch_to_profile_already_set() {
        let mut script = Script::new();
        script.switch_to_profile = Some(SwitchToProfile::Other);

        let result = parse_switch_to_profile("switch:saved", &mut script);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_reboot_action() {
        let cases = [
            ("reboot", RebootAction::Reboot),
            ("shutdown", RebootAction::Shutdown),
        ];

        for (arg, expected) in cases {
            let mut script = Script::new();

            let result = parse_reboot_action(arg, &mut script);

            assert_eq!(result, Ok(true), "Result for argument \"{arg}\"");
            assert_eq!(script.reboot_action, Some(expected));
        }
    }

    #[test]
    fn test_parse_reboot_action_no_switch_arg() {
        let mut script = Script::new();
        let arg = "blah";

        let result = parse_reboot_action(arg, &mut script);

        assert_eq!(result, Ok(false), "Result for argument \"{arg}\"");
    }

    #[test]
    fn test_parse_reboot_action_already_set() {
        let mut script = Script::new();
        script.reboot_action = Some(RebootAction::Reboot);

        let result = parse_reboot_action("shutdown", &mut script);

        assert!(result.is_err());
    }
}
