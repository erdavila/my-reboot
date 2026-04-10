use std::{mem, ptr};

use windows::Win32::Graphics::Gdi::{
    DISPLAY_DEVICE_ACTIVE, DISPLAY_DEVICE_ATTACHED_TO_DESKTOP, DISPLAY_DEVICE_MIRRORING_DRIVER,
    DISPLAY_DEVICE_MODESPRUNED, DISPLAY_DEVICE_PRIMARY_DEVICE, DISPLAY_DEVICE_REMOVABLE,
    DISPLAY_DEVICE_STATE_FLAGS, DISPLAY_DEVICE_VGA_COMPATIBLE, DISPLAY_DEVICEW,
    EnumDisplayDevicesW,
};
use windows::Win32::UI::WindowsAndMessaging::EDD_GET_DEVICE_INTERFACE_NAME;
use windows::core::PCWSTR;

pub struct DisplayDeviceFlags(DISPLAY_DEVICE_STATE_FLAGS);
impl DisplayDeviceFlags {
    #[must_use]
    pub fn active(&self) -> bool {
        self.0.contains(DISPLAY_DEVICE_ACTIVE)
    }
    #[must_use]
    pub fn attached_to_desktop(&self) -> bool {
        self.0.contains(DISPLAY_DEVICE_ATTACHED_TO_DESKTOP)
    }
    #[must_use]
    pub fn mirroring_driver(&self) -> bool {
        self.0.contains(DISPLAY_DEVICE_MIRRORING_DRIVER)
    }
    #[must_use]
    pub fn modes_pruned(&self) -> bool {
        self.0.contains(DISPLAY_DEVICE_MODESPRUNED)
    }
    #[must_use]
    pub fn primary_device(&self) -> bool {
        self.0.contains(DISPLAY_DEVICE_PRIMARY_DEVICE)
    }
    #[must_use]
    pub fn removable(&self) -> bool {
        self.0.contains(DISPLAY_DEVICE_REMOVABLE)
    }
    #[must_use]
    pub fn vga_compatible(&self) -> bool {
        self.0.contains(DISPLAY_DEVICE_VGA_COMPATIBLE)
    }
}

pub struct DisplayDevice {
    pub device_name: String,
    pub device_string: String,
    pub state_flags: DisplayDeviceFlags,
    pub device_id: String,
    pub device_key: String,
}
impl From<&DISPLAY_DEVICEW> for DisplayDevice {
    fn from(dd: &DISPLAY_DEVICEW) -> Self {
        DisplayDevice {
            device_name: u16string_to_string(&dd.DeviceName),
            device_string: u16string_to_string(&dd.DeviceString),
            state_flags: DisplayDeviceFlags(dd.StateFlags),
            device_id: u16string_to_string(&dd.DeviceID),
            device_key: u16string_to_string(&dd.DeviceKey),
        }
    }
}

pub struct EnumDisplayDevices {
    device: Option<Vec<u16>>,
    flags: u32,
    next_index: Option<u32>,
}
impl EnumDisplayDevices {
    #[must_use]
    pub fn new(device: Option<String>, get_device_interface_name: bool) -> EnumDisplayDevices {
        EnumDisplayDevices {
            device: device.map(|dev| {
                let mut dev: Vec<_> = dev.encode_utf16().collect();
                dev.push(0);
                dev
            }),
            flags: if get_device_interface_name {
                EDD_GET_DEVICE_INTERFACE_NAME
            } else {
                0
            },
            next_index: Some(0),
        }
    }

    fn device(&self) -> PCWSTR {
        let ptr = match self.device.as_ref() {
            Some(device) => device.as_ptr().cast(),
            None => ptr::null(),
        };
        PCWSTR::from_raw(ptr)
    }
}
impl Iterator for EnumDisplayDevices {
    type Item = DisplayDevice;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_index {
            #[expect(clippy::cast_possible_truncation)]
            Some(index) => {
                let mut dd = DISPLAY_DEVICEW::default();
                dd.cb = mem::size_of_val(&dd) as u32;

                let result =
                    unsafe { EnumDisplayDevicesW(self.device(), index, &raw mut dd, self.flags) };
                if result.as_bool() {
                    self.next_index = Some(index + 1);
                    Some(DisplayDevice::from(&dd))
                } else {
                    self.next_index = None;
                    None
                }
            }
            None => None,
        }
    }
}

fn u16string_to_string(u16string: &[u16]) -> String {
    let len = u16string
        .iter()
        .position(|&c| c == 0)
        .unwrap_or(u16string.len());
    String::from_utf16_lossy(&u16string[..len])
}
