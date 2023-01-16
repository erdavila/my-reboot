mod get_active_display_id;

pub const STATE_DIR_PATH: &str = "C:\\grubenv.dir";

pub fn get_active_display_id() -> Option<String> {
    Some(get_active_display_id::get_active_display_id())
}
