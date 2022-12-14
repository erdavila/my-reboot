use std::ptr;

pub fn enumerate() {
    enumerate_recursive(ptr::null(), true, "");
}

#[cfg(not(windows))]
fn enumerate_recursive(_: *const u8, _: bool, _: &str) {
    println!("Not on Windows")
}

#[cfg(windows)]
fn enumerate_recursive(name: ::windows_sys::core::PCSTR, recurse: bool, indent: &str) {
    use std::{str::from_utf8, mem::size_of};
    use windows_sys::Win32::Graphics::Gdi::DISPLAY_DEVICEA;
    use windows_sys::Win32::Graphics::Gdi::DISPLAY_DEVICE_ACTIVE;
    use windows_sys::Win32::Graphics::Gdi::EnumDisplayDevicesA;

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
                println!("{}Name: {}", indent, from_utf8(&dd.DeviceName).unwrap());
                println!("{}String: {}", indent, from_utf8(&dd.DeviceString).unwrap());
                println!("{}ID: {}", indent, from_utf8(&dd.DeviceID).unwrap());
                println!("{}Key: {}", indent, from_utf8(&dd.DeviceKey).unwrap());
                println!("{}Active: {}", indent, (dd.StateFlags & DISPLAY_DEVICE_ACTIVE) != 0);

                if recurse {
                    println!("{}Monitors:", indent);

                    let mut i = String::from(indent);
                    i += "  ";

                    enumerate_recursive(dd.DeviceName.as_ptr(), false, &i);
                }

                println!();
            }
        }

        i += 1;
    }
}
