use std::ffi::CString;
use std::ffi::NulError;
use std::mem;
use std::ptr;
use windows_sys::Win32::Graphics::Gdi::EnumDisplayDevicesA;
use windows_sys::Win32::Graphics::Gdi::DISPLAY_DEVICEA;
use windows_sys::Win32::Graphics::Gdi::DISPLAY_DEVICE_ACTIVE;
use windows_sys::Win32::Graphics::Gdi::DISPLAY_DEVICE_MIRRORING_DRIVER;
use windows_sys::Win32::Graphics::Gdi::DISPLAY_DEVICE_MODESPRUNED;
use windows_sys::Win32::Graphics::Gdi::DISPLAY_DEVICE_PRIMARY_DEVICE;
use windows_sys::Win32::Graphics::Gdi::DISPLAY_DEVICE_REMOVABLE;
use windows_sys::Win32::Graphics::Gdi::DISPLAY_DEVICE_VGA_COMPATIBLE;
use windows_sys::Win32::UI::WindowsAndMessaging::EDD_GET_DEVICE_INTERFACE_NAME;

pub struct DisplayDeviceFlags {
    value: u32,
}
impl DisplayDeviceFlags {
    pub fn active(&self) -> bool {
        (self.value & DISPLAY_DEVICE_ACTIVE) != 0
    }
    pub fn mirroring_driver(&self) -> bool {
        (self.value & DISPLAY_DEVICE_MIRRORING_DRIVER) != 0
    }
    pub fn modes_pruned(&self) -> bool {
        (self.value & DISPLAY_DEVICE_MODESPRUNED) != 0
    }
    pub fn primary_device(&self) -> bool {
        (self.value & DISPLAY_DEVICE_PRIMARY_DEVICE) != 0
    }
    pub fn removable(&self) -> bool {
        (self.value & DISPLAY_DEVICE_REMOVABLE) != 0
    }
    pub fn vga_compatible(&self) -> bool {
        (self.value & DISPLAY_DEVICE_VGA_COMPATIBLE) != 0
    }
}

pub struct DisplayDevice {
    pub device_name: CString,
    pub device_string: CString,
    pub state_flags: DisplayDeviceFlags,
    pub device_id: CString,
    pub device_key: CString,
}
impl DisplayDevice {
    fn from(dd: &DISPLAY_DEVICEA) -> Result<DisplayDevice, NulError> {
        Ok(DisplayDevice {
            device_name: byte_slice_to_cstring(&dd.DeviceName)?,
            device_string: byte_slice_to_cstring(&dd.DeviceString)?,
            state_flags: DisplayDeviceFlags {
                value: dd.StateFlags,
            },
            device_id: byte_slice_to_cstring(&dd.DeviceID)?,
            device_key: byte_slice_to_cstring(&dd.DeviceKey)?,
        })
    }
}

pub struct EnumDisplayDevices {
    device: Option<CString>,
    flags: u32,
    next_index: Option<u32>,
}
impl EnumDisplayDevices {
    pub fn new(device: Option<CString>, get_device_interface_name: bool) -> EnumDisplayDevices {
        EnumDisplayDevices {
            device,
            flags: if get_device_interface_name {
                EDD_GET_DEVICE_INTERFACE_NAME
            } else {
                0
            },
            next_index: Some(0),
        }
    }

    fn device(&self) -> *const u8 {
        match self.device.as_ref() {
            Some(device) => device.as_ptr() as *const u8,
            None => ptr::null(),
        }
    }
}
impl Iterator for EnumDisplayDevices {
    type Item = DisplayDevice;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_index {
            Some(index) => {
                let mut dd: DISPLAY_DEVICEA = unsafe { mem::zeroed() };
                dd.cb = mem::size_of_val(&dd) as u32;

                let r = unsafe { EnumDisplayDevicesA(self.device(), index, &mut dd, self.flags) };
                let result = r != 0;

                if result {
                    self.next_index = Some(index + 1);
                    Some(DisplayDevice::from(&dd).unwrap())
                } else {
                    self.next_index = None;
                    None
                }
            }
            None => None,
        }
    }
}

fn byte_slice_to_cstring(bytes: &[u8]) -> Result<CString, NulError> {
    let nul_index = bytes
        .iter()
        .enumerate()
        .find_map(|(i, b)| if *b == 0 { Some(i) } else { None });
    let bytes = match nul_index {
        Some(i) => &bytes[..i],
        None => bytes,
    };
    CString::new(bytes)
}
