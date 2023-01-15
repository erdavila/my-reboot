use std::mem;
use std::ptr;
use std::str;
use windows_sys::Win32::Graphics::Gdi::EnumDisplayDevicesA;
use windows_sys::Win32::Graphics::Gdi::DISPLAY_DEVICEA;
use windows_sys::Win32::Graphics::Gdi::DISPLAY_DEVICE_ACTIVE;

pub fn enumerate() {
    for dd1 in EnumDisplayDevices::new(ptr::null()) {
        print_data(&dd1, 0);

        println!("Monitors:");
        for dd2 in EnumDisplayDevices::new(dd1.DeviceName.as_ptr()) {
            print_data(&dd2, 1);
            println!();
        }
        println!();
    }
}

fn print_data(dd: &DISPLAY_DEVICEA, indent_level: u8) {
    let indent = "  ".repeat(indent_level.into());

    println!(
        "{}Name: {}",
        indent,
        str::from_utf8(&dd.DeviceName).unwrap()
    );
    println!(
        "{}String: {}",
        indent,
        str::from_utf8(&dd.DeviceString).unwrap()
    );
    println!("{}ID: {}", indent, str::from_utf8(&dd.DeviceID).unwrap());
    println!("{}Key: {}", indent, str::from_utf8(&dd.DeviceKey).unwrap());
    println!(
        "{}Active: {}",
        indent,
        (dd.StateFlags & DISPLAY_DEVICE_ACTIVE) != 0
    );
}

pub struct EnumDisplayDevices {
    device: ::windows_sys::core::PCSTR,
    next_index: Option<u32>,
}
impl EnumDisplayDevices {
    pub fn new(device: ::windows_sys::core::PCSTR) -> EnumDisplayDevices {
        EnumDisplayDevices {
            device,
            next_index: Some(0),
        }
    }
}
impl Iterator for EnumDisplayDevices {
    type Item = DISPLAY_DEVICEA;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_index {
            Some(index) => {
                let mut dd = DISPLAY_DEVICEA {
                    cb: mem::size_of::<DISPLAY_DEVICEA>() as u32,
                    DeviceName: [0; 32],
                    DeviceString: [0; 128],
                    StateFlags: 0,
                    DeviceID: [0; 128],
                    DeviceKey: [0; 128],
                };

                let r = unsafe { EnumDisplayDevicesA(self.device, index, &mut dd, 0) };
                let result = r != 0;
                if result {
                    self.next_index = Some(index + 1);
                    Some(dd)
                } else {
                    self.next_index = None;
                    None
                }
            }
            None => None,
        }
    }
}
