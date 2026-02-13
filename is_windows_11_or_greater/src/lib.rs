use std::fmt::Display;

use windows_sys::Wdk::System::SystemServices::KUSER_SHARED_DATA;

const USER_SHARED_DATA_POINTER: *const KUSER_SHARED_DATA = 0x7FFE_0000 as _;

#[must_use]
pub fn is_windows_11_or_greater() -> bool {
    let version = WindowsVersion::get();
    version.is_windows_11_or_greater()
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct WindowsVersion {
    major: u32,
    minor: u32,
    build: u32,
}
impl WindowsVersion {
    #[must_use]
    fn new(major: u32, minor: u32, build: u32) -> Self {
        WindowsVersion {
            major,
            minor,
            build,
        }
    }

    #[must_use]
    pub fn get() -> Self {
        let user_shared_data = unsafe { &*USER_SHARED_DATA_POINTER as &KUSER_SHARED_DATA };
        WindowsVersion::new(
            user_shared_data.NtMajorVersion,
            user_shared_data.NtMinorVersion,
            user_shared_data.NtBuildNumber,
        )
    }

    #[must_use]
    pub fn is_windows_11_or_greater(&self) -> bool {
        *self >= WindowsVersion::new(10, 0, 22000)
    }
}
impl Display for WindowsVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.build)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(!WindowsVersion::new(8, 2, 23000).is_windows_11_or_greater());
        assert!(!WindowsVersion::new(10, 0, 21000).is_windows_11_or_greater());
        assert!(WindowsVersion::new(10, 0, 22000).is_windows_11_or_greater());
        assert!(WindowsVersion::new(10, 0, 23000).is_windows_11_or_greater());
        assert!(WindowsVersion::new(10, 2, 21000).is_windows_11_or_greater());
        assert!(WindowsVersion::new(12, 0, 21000).is_windows_11_or_greater());
    }
}
