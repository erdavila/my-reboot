use crate::options_types::{OperatingSystem, OptionType};
use crate::script::{Script, SetOrUnset};

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
    let processed = parse_next_boot_operating_system(arg, script)?;
    Ok(processed)
}

fn parse_next_boot_operating_system(arg: &str, script: &mut Script) -> Result<bool, ArgError> {
    static OS_PREFIX: &str = "os:";

    fn from_string(os_string: &str) -> Option<SetOrUnset<OperatingSystem>> {
        OperatingSystem::from_option_string(os_string).map(SetOrUnset::Set)
    }

    let os = if let Some(os_string) = arg.strip_prefix(OS_PREFIX) {
        if os_string == UNSET_OPTION {
            Some(SetOrUnset::Unset)
        } else {
            let os = from_string(os_string);
            if os.is_none() {
                return errors::unknown_argument_error(arg);
            }
            os
        }
    } else {
        let os = from_string(arg);
        if os.is_none() {
            return Ok(false);
        }
        os
    };

    if script.next_boot_operating_system.is_none() {
        script.next_boot_operating_system = os;
        Ok(true)
    } else {
        repeated_option_error("sistema operacional", arg)
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
    fn test_parse_multiple_args_invalid() {
        let arg = "os:windows";
        let mut args = ["blah".to_string()].into_iter();

        let result = parse(arg, &mut args);

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
            let mut script = Script {
                next_boot_operating_system: None,
            };

            let result = parse_next_boot_operating_system(arg, &mut script)
                .expect(&format!("Result for argument \"{arg}\" should be Ok(_)"));
            assert!(result);
            assert_eq!(script.next_boot_operating_system, Some(expected));
        }
    }

    #[test]
    fn test_parse_next_boot_operating_system_invalid() {
        let mut script = Script {
            next_boot_operating_system: None,
        };

        let result = parse_next_boot_operating_system("os:blah", &mut script);

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_next_boot_operating_system_not_os_arg() {
        let mut script = Script {
            next_boot_operating_system: None,
        };
        let arg = "blah".to_string();

        let result = parse_next_boot_operating_system(&arg, &mut script);

        let result = result.expect(&format!("Result for argument \"{arg}\" should be Ok(_)"));
        assert!(!result);
    }

    #[test]
    fn test_parse_next_boot_operating_system_already_set() {
        let mut script = Script {
            next_boot_operating_system: Some(Unset),
        };

        let result = parse_next_boot_operating_system("os:windows", &mut script);

        assert!(result.is_err());
    }
}
