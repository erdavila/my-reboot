use is_windows_11_or_greater::WindowsVersion;

fn main() {
    let version = WindowsVersion::get();
    println!("{version}");
    println!("{}", version.is_windows_11_or_greater());
}
