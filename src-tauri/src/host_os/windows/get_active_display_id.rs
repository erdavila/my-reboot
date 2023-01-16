use enum_display_devices::EnumDisplayDevices;

pub fn get_active_display_id() -> String {
    for dd1 in EnumDisplayDevices::new(None, false) {
        for dd2 in EnumDisplayDevices::new(Some(dd1.device_name), false) {
            if dd2.state_flags.active() {
                return String::from_utf8(dd2.device_id.into_bytes()).unwrap();
            }
        }
    }

    panic!("Não foi possível identificar a tela atual");
}
