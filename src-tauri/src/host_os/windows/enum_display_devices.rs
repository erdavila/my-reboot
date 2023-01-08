use std::mem::size_of;
use std::ptr;
use std::str::from_utf8;
use windows_sys::Win32::Graphics::Gdi::EnumDisplayDevicesA;
use windows_sys::Win32::Graphics::Gdi::DISPLAY_DEVICEA;
use windows_sys::Win32::Graphics::Gdi::DISPLAY_DEVICE_ACTIVE;

pub fn enumerate() {
    enumerate_recursive(ptr::null(), true, 0)
}

fn enumerate_recursive(name: ::windows_sys::core::PCSTR, recurse: bool, indent_level: u8) {
    let mut dd = DISPLAY_DEVICEA {
        cb: size_of::<DISPLAY_DEVICEA>() as u32,
        DeviceName: [0; 32],
        DeviceString: [0; 128],
        StateFlags: 0,
        DeviceID: [0; 128],
        DeviceKey: [0; 128],
    };

    let mut result = true;
    let mut i = 0;
    while result {
        unsafe {
            let r = EnumDisplayDevicesA(name, i, &mut dd, 0);
            result = r != 0;

            if result {
                let indent = "  ".repeat(indent_level.into());

                println!("{}Name: {}", indent, from_utf8(&dd.DeviceName).unwrap());
                println!("{}String: {}", indent, from_utf8(&dd.DeviceString).unwrap());
                println!("{}ID: {}", indent, from_utf8(&dd.DeviceID).unwrap());
                println!("{}Key: {}", indent, from_utf8(&dd.DeviceKey).unwrap());
                println!(
                    "{}Active: {}",
                    indent,
                    (dd.StateFlags & DISPLAY_DEVICE_ACTIVE) != 0
                );

                if recurse {
                    println!("{}Monitors:", indent);
                    enumerate_recursive(dd.DeviceName.as_ptr(), false, indent_level + 1);
                }

                println!();
            }
        }

        i += 1;
    }
}
