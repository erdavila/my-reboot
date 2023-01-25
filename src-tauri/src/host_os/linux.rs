use crate::configs::Configs;

use super::CurrentDisplayHandler;

pub const STATE_DIR_PATH: &str = "/boot/grub/grubenv.dir";

pub fn get_current_display_handler<'a>(
    _: &'a Configs,
) -> Option<Box<dyn CurrentDisplayHandler + 'a>> {
    None
}
