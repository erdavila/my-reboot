use anyhow::Result;
use windows::Win32::Devices::Display as WinDisplay;
use windows::Win32::Foundation as WinFoundation;

use crate::win_api::functions::win32_error::Win32Error;
use crate::win_api::types::device_id::DeviceId;
use crate::win_api::types::flags::{Flags, impl_win_flags_for};
use crate::win_api::types::lower_hex::impl_to_lower_hex_for_newtype;
use crate::win_api::types::{self, define_flag_type};

mod win32_error;

pub trait DeviceInfo {
    const TYPE: WinDisplay::DISPLAYCONFIG_DEVICE_INFO_TYPE;

    fn header(&mut self) -> &mut WinDisplay::DISPLAYCONFIG_DEVICE_INFO_HEADER;
}
macro_rules! impl_device_info_for {
    ($type:ident , $type_const:ident $(,)?) => {
        impl DeviceInfo for WinDisplay::$type {
            const TYPE: WinDisplay::DISPLAYCONFIG_DEVICE_INFO_TYPE = WinDisplay::$type_const;

            fn header(&mut self) -> &mut WinDisplay::DISPLAYCONFIG_DEVICE_INFO_HEADER {
                &mut self.header
            }
        }
    };
}
impl_device_info_for!(
    DISPLAYCONFIG_SOURCE_DEVICE_NAME,
    DISPLAYCONFIG_DEVICE_INFO_GET_SOURCE_NAME,
);
impl_device_info_for!(
    DISPLAYCONFIG_TARGET_DEVICE_NAME,
    DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_NAME,
);
/*
DISPLAYCONFIG_TARGET_PREFERRED_MODE
DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_PREFERRED_MODE
*/
impl_device_info_for!(
    DISPLAYCONFIG_ADAPTER_NAME,
    DISPLAYCONFIG_DEVICE_INFO_GET_ADAPTER_NAME,
);
/*
DISPLAYCONFIG_TARGET_BASE_TYPE
DISPLAYCONFIG_DEVICE_INFO_GET_TARGET_BASE_TYPE

DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION
DISPLAYCONFIG_DEVICE_INFO_GET_SUPPORT_VIRTUAL_RESOLUTION

DISPLAYCONFIG_SUPPORT_VIRTUAL_RESOLUTION
DISPLAYCONFIG_DEVICE_INFO_SET_SUPPORT_VIRTUAL_RESOLUTION

???
DISPLAYCONFIG_DEVICE_INFO_GET_ADVANCED_COLOR_INFO

DISPLAYCONFIG_SDR_WHITE_LEVEL
DISPLAYCONFIG_DEVICE_INFO_GET_SDR_WHITE_LEVEL
*/

#[expect(clippy::cast_possible_truncation)]
pub fn display_config_get_device_info<T: DeviceInfo + Default>(device_id: DeviceId) -> Result<T> {
    let mut device_info = T::default();
    *device_info.header() = WinDisplay::DISPLAYCONFIG_DEVICE_INFO_HEADER {
        r#type: T::TYPE,
        size: std::mem::size_of::<T>() as u32,
        adapterId: device_id.adapterId.into(),
        id: device_id.id,
    };

    let error = unsafe { WinDisplay::DisplayConfigGetDeviceInfo(device_info.header()) };
    Win32Error::from(error).into_result(device_info)
}

define_flag_type!(
    QUERY_DISPLAY_CONFIG_FLAG : WinDisplay::QUERY_DISPLAY_CONFIG_FLAGS {
        WinDisplay;
        QDC_ALL_PATHS => ALL_PATHS,
        QDC_ONLY_ACTIVE_PATHS => ONLY_ACTIVE_PATHS,
        QDC_DATABASE_CURRENT => DATABASE_CURRENT,
        QDC_VIRTUAL_MODE_AWARE => VIRTUAL_MODE_AWARE,
        QDC_INCLUDE_HMD => INCLUDE_HMD,
        QDC_VIRTUAL_REFRESH_RATE_AWARE => VIRTUAL_REFRESH_RATE_AWARE,
    }
);
impl_win_flags_for!(WinDisplay::QUERY_DISPLAY_CONFIG_FLAGS);
impl_to_lower_hex_for_newtype!(WinDisplay::QUERY_DISPLAY_CONFIG_FLAGS);

pub fn query_display_config(
    flags: Flags<QUERY_DISPLAY_CONFIG_FLAG>,
) -> Result<(
    Vec<types::DISPLAYCONFIG_PATH_INFO>,
    Vec<types::DISPLAYCONFIG_MODE_INFO>,
)> {
    let mut numpatharrayelements = 0u32;
    let mut nummodeinfoarrayelements = 0u32;

    loop {
        let error = unsafe {
            WinDisplay::GetDisplayConfigBufferSizes(
                flags.into(),
                &raw mut numpatharrayelements,
                &raw mut nummodeinfoarrayelements,
            )
        };
        Win32Error(error).into_unit_result()?;

        let mut patharray =
            vec![WinDisplay::DISPLAYCONFIG_PATH_INFO::default(); numpatharrayelements as usize];
        let mut modeinfoarray =
            vec![WinDisplay::DISPLAYCONFIG_MODE_INFO::default(); nummodeinfoarrayelements as usize];

        let error = unsafe {
            WinDisplay::QueryDisplayConfig(
                flags.into(),
                &raw mut numpatharrayelements,
                patharray.as_mut_ptr(),
                &raw mut nummodeinfoarrayelements,
                modeinfoarray.as_mut_ptr(),
                None,
            )
        };

        if error == WinFoundation::ERROR_INSUFFICIENT_BUFFER {
            // Try again
            continue;
        }

        return Win32Error(error).into_result_with(|| {
            let paths = patharray
                .into_iter()
                .take(numpatharrayelements as usize)
                .map(Into::into)
                .collect();
            let modes = modeinfoarray
                .into_iter()
                .take(nummodeinfoarrayelements as usize)
                .map(Into::into)
                .collect();

            (paths, modes)
        });
    }
}
