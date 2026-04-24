use std::mem;

use windows::Win32::Devices::Display::{
    DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME, DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
    DISPLAYCONFIG_DEVICE_INFO_HEADER, DISPLAYCONFIG_DEVICE_INFO_TYPE, DISPLAYCONFIG_MODE_INFO,
    DISPLAYCONFIG_PATH_INFO, DISPLAYCONFIG_SOURCE_DEVICE_NAME, DISPLAYCONFIG_TARGET_DEVICE_NAME,
    DisplayConfigGetDeviceInfo, GetDisplayConfigBufferSizes, QDC_VIRTUAL_MODE_AWARE,
    QUERY_DISPLAY_CONFIG_FLAGS, QueryDisplayConfig,
};
use windows::Win32::Foundation::{ERROR_INSUFFICIENT_BUFFER, LUID};

use crate::Result;
use crate::win_api::win32_error::Win32Error;

pub mod win32_error;

pub fn query_display_config(
    mut flags: QUERY_DISPLAY_CONFIG_FLAGS,
) -> Result<(Vec<DISPLAYCONFIG_PATH_INFO>, Vec<DISPLAYCONFIG_MODE_INFO>)> {
    flags |= QDC_VIRTUAL_MODE_AWARE;

    loop {
        let mut path_count = 0;
        let mut mode_count = 0;

        let error =
            unsafe { GetDisplayConfigBufferSizes(flags, &raw mut path_count, &raw mut mode_count) };
        Win32Error::from(error).to_result("GetDisplayConfigBufferSizes", ())?;

        let mut paths = vec![DISPLAYCONFIG_PATH_INFO::default(); path_count as usize];
        let mut modes = vec![DISPLAYCONFIG_MODE_INFO::default(); mode_count as usize];

        let error = unsafe {
            QueryDisplayConfig(
                flags,
                &raw mut path_count,
                paths.as_mut_ptr(),
                &raw mut mode_count,
                modes.as_mut_ptr(),
                None,
            )
        };

        if error != ERROR_INSUFFICIENT_BUFFER {
            return Win32Error::from(error).to_result_with("QueryDisplayConfig", || {
                paths.truncate(path_count as usize);
                modes.truncate(mode_count as usize);
                (paths, modes)
            });
        }

        // Try again.
    }
}

pub trait GetDeviceInfo: Default {
    const TYPE: DISPLAYCONFIG_DEVICE_INFO_TYPE;
    fn header(&mut self) -> &mut DISPLAYCONFIG_DEVICE_INFO_HEADER;
}

impl GetDeviceInfo for DISPLAYCONFIG_TARGET_DEVICE_NAME {
    const TYPE: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME;
    fn header(&mut self) -> &mut DISPLAYCONFIG_DEVICE_INFO_HEADER {
        &mut self.header
    }
}

impl GetDeviceInfo for DISPLAYCONFIG_SOURCE_DEVICE_NAME {
    const TYPE: DISPLAYCONFIG_DEVICE_INFO_TYPE = DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME;
    fn header(&mut self) -> &mut DISPLAYCONFIG_DEVICE_INFO_HEADER {
        &mut self.header
    }
}

pub fn display_config_get_device_info<T: GetDeviceInfo>(adapter_id: LUID, id: u32) -> Result<T> {
    let mut device_info = T::default();
    *device_info.header() = DISPLAYCONFIG_DEVICE_INFO_HEADER {
        r#type: T::TYPE,
        #[expect(clippy::cast_possible_truncation)]
        size: mem::size_of::<T>() as u32,
        adapterId: adapter_id,
        id,
    };

    let error = unsafe { DisplayConfigGetDeviceInfo(device_info.header()) };
    Win32Error::from(error).to_result("DisplayConfigGetDeviceInfo", device_info)
}
