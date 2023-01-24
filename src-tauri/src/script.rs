use crate::options_types::OperatingSystem;
use crate::state::StateProvider;
use crate::text;
use crate::text::NEXT_BOOT_OPERATING_SYSTEM_SENTENCE;

pub struct Script {
    pub next_boot_operating_system: Option<SetOrUnset<OperatingSystem>>,
}
impl Script {
    pub fn new() -> Self {
        Script {
            next_boot_operating_system: None,
        }
    }

    pub fn execute(&self) {
        let mut state_provider = StateProvider::new().unwrap();
        use SetOrUnset::*;

        if let Some(os_option) = &self.next_boot_operating_system {
            match os_option {
                Set(os) => state_provider.set_next_boot_operating_system(*os),
                Unset => state_provider.unset_next_boot_operating_system(),
            }

            println!(
                "{NEXT_BOOT_OPERATING_SYSTEM_SENTENCE} foi atualizado para {}.",
                text::operating_system_text(os_option.to_option())
            );
        }
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
