use std::collections::BTreeMap;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::get_device_info::{
    SourceDeviceInfos, TargetDeviceInfos, get_source_device_infos, get_target_device_infos,
};
use crate::win_api::types;
use crate::win_api::types::device_id::DeviceId;
use crate::win_api::types::flags::Flags;

pub type Configs = Vec<AggregatedPath>;

pub struct DisaggregatedConfigs {
    pub paths: Vec<types::DISPLAYCONFIG_PATH_INFO>,
    pub modes: Vec<types::DISPLAYCONFIG_MODE_INFO>,
    pub source_device_infos: BTreeMap<DeviceId, SourceDeviceInfos>,
    pub target_device_infos: BTreeMap<DeviceId, TargetDeviceInfos>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AggregatedPath {
    pub flags: Flags<types::DISPLAYCONFIG_PATH_INFO_flag>,

    pub sourceInfo: types::DISPLAYCONFIG_PATH_SOURCE_INFO,
    pub sourceMode: Option<types::DISPLAYCONFIG_SOURCE_MODE>,
    pub sourceDeviceInfos: SourceDeviceInfos,

    pub targetInfo: types::DISPLAYCONFIG_PATH_TARGET_INFO,
    pub targetMode: Option<types::DISPLAYCONFIG_TARGET_MODE>,
    pub targetDeviceInfos: TargetDeviceInfos,

    pub desktopMode: Option<types::DISPLAYCONFIG_DESKTOP_IMAGE_INFO>,
}

pub fn aggregate_configs(
    paths: Vec<types::DISPLAYCONFIG_PATH_INFO>,
    modes: Vec<types::DISPLAYCONFIG_MODE_INFO>,
) -> Result<Vec<AggregatedPath>> {
    let configs = paths
        .into_iter()
        .map(|path| {
            let sourceMode = path
                .source_mode_idx()
                .map(|idx| match modes[idx].Anonymous {
                    types::DISPLAYCONFIG_MODE_INFO_0::sourceMode(sourceMode) => sourceMode,
                    _ => unreachable!(),
                });
            let targetMode = path
                .target_mode_idx()
                .map(|idx| match modes[idx].Anonymous {
                    types::DISPLAYCONFIG_MODE_INFO_0::targetMode(targetMode) => targetMode,
                    _ => unreachable!(),
                });
            let desktopMode = path
                .desktop_mode_idx()
                .map(|idx| match modes[idx].Anonymous {
                    types::DISPLAYCONFIG_MODE_INFO_0::desktopImageInfo(desktopMode) => desktopMode,
                    _ => unreachable!(),
                });

            let sourceDeviceInfos = get_source_device_infos(path.sourceInfo.device_id)?;
            let targetDeviceInfos = get_target_device_infos(path.targetInfo.device_id)?;

            Ok(AggregatedPath {
                flags: path.flags,
                sourceInfo: path.sourceInfo,
                sourceMode,
                sourceDeviceInfos,
                targetInfo: path.targetInfo,
                targetMode,
                targetDeviceInfos,
                desktopMode,
            })
        })
        .collect();

    drop(modes);

    configs
}

#[must_use]
pub fn disaggregate_configs(configs: Configs) -> DisaggregatedConfigs {
    let mut paths = Vec::new();
    let mut modes = Vec::new();
    let mut source_device_infos = BTreeMap::new();
    let mut target_device_infos = BTreeMap::new();

    let mut add_mode = |device_id, Anonymous| {
        let idx = modes.len();
        modes.push(types::DISPLAYCONFIG_MODE_INFO {
            device_id,
            Anonymous,
        });
        idx
    };

    for aggregated_path in configs {
        let mut path = types::DISPLAYCONFIG_PATH_INFO {
            sourceInfo: aggregated_path.sourceInfo,
            targetInfo: aggregated_path.targetInfo,
            flags: aggregated_path.flags,
        };

        if let Some(source_mode) = aggregated_path.sourceMode {
            let idx = add_mode(
                path.sourceInfo.device_id,
                types::DISPLAYCONFIG_MODE_INFO_0::sourceMode(source_mode),
            );
            path.set_source_mode_idx(Some(idx));
        }
        if let Some(target_mode) = aggregated_path.targetMode {
            let idx = add_mode(
                path.targetInfo.device_id,
                types::DISPLAYCONFIG_MODE_INFO_0::targetMode(target_mode),
            );
            path.set_target_mode_idx(Some(idx));
        }
        if let Some(desktop_info) = aggregated_path.desktopMode {
            let idx = add_mode(
                path.targetInfo.device_id,
                types::DISPLAYCONFIG_MODE_INFO_0::desktopImageInfo(desktop_info),
            );
            path.set_desktop_mode_idx(Some(idx));
        }

        source_device_infos.insert(path.sourceInfo.device_id, aggregated_path.sourceDeviceInfos);
        target_device_infos.insert(path.targetInfo.device_id, aggregated_path.targetDeviceInfos);

        paths.push(path);
    }

    DisaggregatedConfigs {
        paths,
        modes,
        source_device_infos,
        target_device_infos,
    }
}
