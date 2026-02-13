use crate::options_types::{Display, OptionType, RebootAction};
use crate::script::{Script, SetOrUnset, SwitchToDisplay};
use crate::text;

use super::errors::{self, ArgError};

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

static UNSET_OPTION: &str = "unset";

fn parse_single(arg: &str, script: &mut Script) -> Result<bool, ArgError> {
    if parse_next_boot_operating_system(arg, script)? {
        return Ok(true);
    }

    if parse_next_windows_boot_display(arg, script)? {
        return Ok(true);
    }

    if parse_switch_to_display(arg, script)? {
        return Ok(true);
    }

    if parse_reboot_action(arg, script)? {
        return Ok(true);
    }

    Ok(false)
}

fn parse_next_boot_operating_system(arg: &str, script: &mut Script) -> Result<bool, ArgError> {
    parse_boot_option(
        arg,
        &mut script.next_boot_operating_system,
        "os:",
        text::operating_system::ON_NEXT_BOOT_DESCRIPTION,
    )
}

fn parse_next_windows_boot_display(arg: &str, script: &mut Script) -> Result<bool, ArgError> {
    parse_boot_option(
        arg,
        &mut script.next_windows_boot_display,
        "display:",
        text::display::ON_NEXT_WINDOWS_BOOT_DESCRIPTION,
    )
}

fn parse_boot_option<T: OptionType>(
    arg: &str,
    value: &mut Option<SetOrUnset<T>>,
    prefix: &str,
    descr: &str,
) -> Result<bool, ArgError> {
    let option = if let Some(string) = arg.strip_prefix(prefix) {
        if string == UNSET_OPTION {
            SetOrUnset::<T>::Unset
        } else {
            match T::from_option_string(string).map(SetOrUnset::Set) {
                Some(option) => option,
                None => return errors::unknown_argument_error(arg),
            }
        }
    } else {
        match T::from_option_string(arg).map(SetOrUnset::Set) {
            Some(option) => option,
            None => return Ok(false),
        }
    };

    set_if_none(value, option, arg, descr)
}

fn parse_switch_to_display(arg: &str, script: &mut Script) -> Result<bool, ArgError> {
    let option = match arg.strip_prefix("switch:") {
        Some("saved") => SwitchToDisplay::Saved,
        Some("other") => SwitchToDisplay::Other,
        Some(string) => match Display::from_option_string(string) {
            Some(display) => SwitchToDisplay::Display(display),
            None => return errors::unknown_argument_error(arg),
        },
        None if arg == "switch" => SwitchToDisplay::Other,
        None => return Ok(false),
    };

    set_if_none(&mut script.switch_to_display, option, arg, "troca de tela")
}

fn parse_reboot_action(arg: &str, script: &mut Script) -> Result<bool, ArgError> {
    let Some(option) = RebootAction::from_option_string(arg) else {
        return Ok(false);
    };

    set_if_none(&mut script.reboot_action, option, arg, "ação")
}

fn set_if_none<T>(
    current: &mut Option<T>,
    new: T,
    arg: &str,
    descr: &str,
) -> Result<bool, ArgError> {
    if current.is_none() {
        current.replace(new);
        Ok(true)
    } else {
        repeated_option_error(descr, arg)
    }
}

fn repeated_option_error<T>(option: &str, arg: &str) -> Result<T, ArgError> {
    Err(ArgError::new(
        &format!("A opção de {option} não pode ser usada mais de uma vez"),
        arg,
    ))
}

#[cfg(test)]
mod tests {
    use std::iter;

    use crate::options_types::{Display, OperatingSystem};
    use crate::script::SwitchToDisplay;

    use super::*;
    use SetOrUnset::*;

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
        let arg = "display:monitor";
        let mut args = ["os:windows".to_string()].into_iter();

        let result = parse(arg, &mut args);

        let option = result.expect("result should be Ok(_)");
        let script = option.expect("option should be Some(_)");
        assert_eq!(
            script.next_boot_operating_system,
            Some(Set(OperatingSystem::Windows))
        );
        assert_eq!(
            script.next_windows_boot_display,
            Some(Set(Display::Monitor))
        );
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
    fn test_parse_single_display() {
        let mut script = Script::new();

        let result = parse_single("display:monitor", &mut script);

        let success = result.expect("result should be Ok(_)");
        assert!(success);
        assert_eq!(
            script.next_windows_boot_display,
            Some(Set(Display::Monitor))
        );
    }

    #[test]
    fn test_parse_single_switch() {
        let mut script = Script::new();

        let result = parse_single("switch:monitor", &mut script);

        let success = result.expect("result should be Ok(_)");
        assert!(success);
        assert_eq!(
            script.switch_to_display,
            Some(SwitchToDisplay::Display(Display::Monitor))
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

        let result = parse_single("display:blah", &mut script);

        assert!(result.is_err());
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

            let result = parse_next_boot_operating_system(arg, &mut script)
                .expect(&format!("Result for argument \"{arg}\" should be Ok(_)"));
            assert!(result);
            assert_eq!(script.next_boot_operating_system, Some(expected));
        }
    }

    #[test]
    fn test_parse_next_boot_operating_system_invalid() {
        let mut script = Script::new();

        let result = parse_next_boot_operating_system("os:blah", &mut script);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_next_boot_operating_system_not_os_arg() {
        let mut script = Script::new();
        let arg = "blah".to_string();

        let result = parse_next_boot_operating_system(&arg, &mut script);

        let success = result.expect(&format!("Result for argument \"{arg}\" should be Ok(_)"));
        assert!(!success);
    }

    #[test]
    fn test_parse_next_boot_operating_system_already_set() {
        let mut script = Script::new();
        script.next_boot_operating_system = Some(Unset);

        let result = parse_next_boot_operating_system("os:windows", &mut script);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_next_windows_boot_display() {
        let cases = [
            ("display:monitor", Set(Display::Monitor)),
            ("monitor", Set(Display::Monitor)),
            ("display:tv", Set(Display::TV)),
            ("tv", Set(Display::TV)),
            ("display:unset", Unset),
        ];

        for (arg, expected) in cases {
            let mut script = Script::new();

            let success = parse_next_windows_boot_display(arg, &mut script)
                .expect(&format!("Result for argument \"{arg}\" should be Ok(_)"));
            assert!(success);
            assert_eq!(script.next_windows_boot_display, Some(expected));
        }
    }

    #[test]
    fn test_parse_next_windows_boot_display_invalid() {
        let mut script = Script::new();

        let result = parse_next_windows_boot_display("display:blah", &mut script);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_next_windows_boot_display_not_display_arg() {
        let mut script = Script::new();
        let arg = "blah".to_string();

        let result = parse_next_windows_boot_display(&arg, &mut script);

        let success = result.expect(&format!("Result for argument \"{arg}\" should be Ok(_)"));
        assert!(!success);
    }

    #[test]
    fn test_parse_next_windows_boot_display_already_set() {
        let mut script = Script::new();
        script.next_windows_boot_display = Some(Unset);

        let result = parse_next_windows_boot_display("display:monitor", &mut script);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_switch_to_display() {
        let cases = [
            ("switch", SwitchToDisplay::Other),
            ("switch:other", SwitchToDisplay::Other),
            ("switch:monitor", SwitchToDisplay::Display(Display::Monitor)),
            ("switch:tv", SwitchToDisplay::Display(Display::TV)),
            ("switch:saved", SwitchToDisplay::Saved),
        ];

        for (arg, expected) in cases {
            let mut script = Script::new();

            let success = parse_switch_to_display(arg, &mut script)
                .expect(&format!("Result for argument \"{arg}\" should be Ok(_)"));
            assert!(success);
            assert_eq!(script.switch_to_display, Some(expected));
        }
    }

    #[test]
    fn test_parse_switch_to_display_invalid() {
        let mut script = Script::new();

        let result = parse_switch_to_display("switch:blah", &mut script);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_switch_to_display_no_switch_arg() {
        let mut script = Script::new();
        let arg = "blah";

        let result = parse_switch_to_display(arg, &mut script);

        let success = result.expect(&format!("Result for argument \"{arg}\" should be Ok(_)"));
        assert!(!success);
    }

    #[test]
    fn test_parse_switch_to_display_already_set() {
        let mut script = Script::new();
        script.switch_to_display = Some(SwitchToDisplay::Other);

        let result = parse_switch_to_display("switch:saved", &mut script);

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

            let success = parse_reboot_action(arg, &mut script)
                .expect(&format!("Result for argument \"{arg}\" should be Ok(_)"));
            assert!(success);
            assert_eq!(script.reboot_action, Some(expected));
        }
    }

    #[test]
    fn test_parse_reboot_action_no_switch_arg() {
        let mut script = Script::new();
        let arg = "blah";

        let result = parse_reboot_action(arg, &mut script);

        let success = result.expect(&format!("Result for argument \"{arg}\" should be Ok(_)"));
        assert!(!success);
    }

    #[test]
    fn test_parse_reboot_action_already_set() {
        let mut script = Script::new();
        script.reboot_action = Some(RebootAction::Reboot);

        let result = parse_reboot_action("shutdown", &mut script);

        assert!(result.is_err());
    }
}
