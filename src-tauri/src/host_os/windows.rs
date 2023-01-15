mod enum_display_devices;
mod get_active_display_id;

pub const STATE_DIR_PATH: &str = "C:\\grubenv.dir";

pub use enum_display_devices::enumerate as enumerate_display_devices;

pub fn get_active_display_id() -> Option<String> {
    Some(get_active_display_id::get_active_display_id())
}
