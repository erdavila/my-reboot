use anyhow::Result;
use serde::Serialize;
use windows::Win32::Devices::Display as WinDisplay;

use crate::win_api::functions;
use crate::win_api::types::device_id::DeviceId;

pub fn get_source_device_infos(device_id: DeviceId) -> Result<SourceDeviceInfos> {
    let device_name = functions::display_config_get_device_info::<
        WinDisplay::DISPLAYCONFIG_SOURCE_DEVICE_NAME,
    >(device_id)?;
    let viewGdiDeviceName = from_windows_string(&device_name.viewGdiDeviceName);

    let adapter_name = functions::display_config_get_device_info::<
        WinDisplay::DISPLAYCONFIG_ADAPTER_NAME,
    >(device_id)?;
    let adapterDevicePath = from_windows_string(&adapter_name.adapterDevicePath);

    Ok(SourceDeviceInfos {
        viewGdiDeviceName,
        adapterDevicePath,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SourceDeviceInfos {
    viewGdiDeviceName: String,
    adapterDevicePath: String,
}

pub fn get_target_device_infos(device_id: DeviceId) -> Result<TargetDeviceInfos> {
    let device_name = functions::display_config_get_device_info::<
        WinDisplay::DISPLAYCONFIG_TARGET_DEVICE_NAME,
    >(device_id)?;
    // pub flags: DISPLAYCONFIG_TARGET_DEVICE_NAME_FLAGS,
    let monitorFriendlyDeviceName = from_windows_string(&device_name.monitorFriendlyDeviceName);
    let monitorDevicePath = from_windows_string(&device_name.monitorDevicePath);
    // pub outputTechnology: DISPLAYCONFIG_VIDEO_OUTPUT_TECHNOLOGY,
    // pub edidManufactureId: u16,
    // pub edidProductCodeId: u16,
    // pub connectorInstance: u32,
    // pub monitorFriendlyDeviceName: [u16; 64],
    // pub monitorDevicePath: [u16; 128],

    let adapter_name = functions::display_config_get_device_info::<
        WinDisplay::DISPLAYCONFIG_ADAPTER_NAME,
    >(device_id)?;
    let adapterDevicePath = from_windows_string(&adapter_name.adapterDevicePath);

    Ok(TargetDeviceInfos {
        monitorFriendlyDeviceName,
        monitorDevicePath,
        adapterDevicePath,
    })
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct TargetDeviceInfos {
    monitorFriendlyDeviceName: String,
    monitorDevicePath: String,
    adapterDevicePath: String,
}

fn from_windows_string(s: &[u16]) -> String {
    let len = s.iter().position(|&c| c == 0).unwrap_or(s.len());
    String::from_utf16_lossy(&s[..len])
}
