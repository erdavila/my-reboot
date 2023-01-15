use std::ptr;
use windows_sys::Win32::Graphics::Gdi::DISPLAY_DEVICE_ACTIVE;

use crate::host_os::windows::enum_display_devices::EnumDisplayDevices;

pub fn get_active_display_id() -> String {
    for dd1 in EnumDisplayDevices::new(ptr::null()) {
        for dd2 in EnumDisplayDevices::new(dd1.DeviceName.as_ptr()) {
            if (dd2.StateFlags & DISPLAY_DEVICE_ACTIVE) != 0 {
                return String::from_utf8(filter_nul_bytes(&dd2.DeviceID)).unwrap();
            }
        }
    }

    panic!("Não foi possível identificar a tela atual");
}

fn filter_nul_bytes(bytes: &[u8]) -> Vec<u8> {
    bytes.iter().take_while(|b| **b != 0).copied().collect()
}
