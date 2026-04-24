use windows::Win32::Devices::Display::{
    DISPLAYCONFIG_SOURCE_DEVICE_NAME, DISPLAYCONFIG_TARGET_DEVICE_NAME, QDC_ONLY_ACTIVE_PATHS,
};
use windows::Win32::Graphics::Gdi::{
    DISPLAYCONFIG_PATH_MODE_IDX_INVALID, DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE,
};

use crate::error::Error;
use crate::win_api::{display_config_get_device_info, query_display_config};
use crate::{Dimensions, Monitor, Profile, Result, is_flag_set, windows_string_to_string};

pub fn get_profile() -> Result<Profile> {
    let (paths, modes) = query_display_config(QDC_ONLY_ACTIVE_PATHS)?;

    paths
        .into_iter()
        .map(|path| {
            let source_device_name = display_config_get_device_info::<
                DISPLAYCONFIG_SOURCE_DEVICE_NAME,
            >(path.sourceInfo.adapterId, path.sourceInfo.id)?;

            let target_device_name = display_config_get_device_info::<
                DISPLAYCONFIG_TARGET_DEVICE_NAME,
            >(path.targetInfo.adapterId, path.targetInfo.id)?;

            let source_mode = unsafe {
                let idx = if is_flag_set(path.flags, DISPLAYCONFIG_PATH_SUPPORT_VIRTUAL_MODE) {
                    path.sourceInfo.Anonymous.Anonymous._bitfield >> 16
                } else {
                    path.sourceInfo.Anonymous.modeInfoIdx
                };

                if idx == DISPLAYCONFIG_PATH_MODE_IDX_INVALID {
                    return Err(Error::Custom(
                        "no source mode in the display path info".to_string(),
                    ));
                }
                &modes[idx as usize].Anonymous.sourceMode
            };

            Ok(Monitor {
                friendly_device_name: windows_string_to_string(
                    &target_device_name.monitorFriendlyDeviceName,
                )?,
                source_device_name: windows_string_to_string(
                    &source_device_name.viewGdiDeviceName,
                )?,
                device_path: windows_string_to_string(&target_device_name.monitorDevicePath)?,
                dimensions: Dimensions {
                    width: source_mode.width,
                    height: source_mode.height,
                },
                pixel_format: source_mode.pixelFormat.into(),
                position: source_mode.position.into(),
                rotation: path.targetInfo.rotation.into(),
                scaling: path.targetInfo.scaling.into(),
                refresh_rate: path.targetInfo.refreshRate.into(),
            })
        })
        .collect()
}
