#[cfg(windows)]
use enum_display_devices::DisplayDevice;

#[cfg(windows)]
use enum_display_devices::EnumDisplayDevices;

#[cfg(windows)]
fn main() {
    for dd1 in EnumDisplayDevices::new(None, false) {
        print_data(&dd1, 0);

        let mut any_monitor = false;
        for dd2 in EnumDisplayDevices::new(Some(dd1.device_name), false) {
            if !any_monitor {
                println!("Monitors:");
                any_monitor = true;
            }

            println!();
            print_data(&dd2, 1);
        }

        println!();
    }
}

#[cfg(windows)]
fn print_data(dd: &DisplayDevice, indent_level: u8) {
    let indent = "  ".repeat(indent_level.into());

    println!("{indent}Name: {}", dd.device_name.to_str().unwrap());
    println!("{indent}String: {}", dd.device_string.to_str().unwrap());
    println!("{indent}ID: {}", &dd.device_id.to_str().unwrap());
    println!("{indent}Key: {}", &dd.device_key.to_str().unwrap());
    println!("{indent}Active: {}", dd.state_flags.active());
}

#[cfg(not(windows))]
fn main() {
    panic!("Disponível somente no Windows");
}
