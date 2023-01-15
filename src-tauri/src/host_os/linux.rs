pub const STATE_DIR_PATH: &str = "/boot/grub/grubenv.dir";

pub fn enumerate_display_devices() {
    println!("Not on Windows");
}

pub fn get_active_display_id() -> Option<String> {
    None
}
