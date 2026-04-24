use std::collections::BTreeSet;
use std::fs::{self, File};
use std::path::PathBuf;

use anyhow::{Result, anyhow, bail};
use display_profile_experiment::common::{
    Configs, DisaggregatedConfigs, aggregate_configs, disaggregate_configs,
};
use display_profile_experiment::win_api::functions::{self, QUERY_DISPLAY_CONFIG_FLAG};
use display_profile_experiment::win_api::types;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::json;

const DATA_DIR_PATH: &str = "display-profile/data";

type Profile = Vec<Monitor>;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
struct Monitor {
    friendly_device_name: String,
    source_device_name: String,
    device_path: String,
    width: u32,
    height: u32,
    pixel_format: types::DISPLAYCONFIG_PIXELFORMAT,
    position: types::POINTL,
    rotation: types::DISPLAYCONFIG_ROTATION,
    scaling: types::DISPLAYCONFIG_SCALING,
    refresh_rate: types::DISPLAYCONFIG_RATIONAL,
}

fn main() -> Result<()> {
    let args = parse_args()?;

    let profile = match args.profile_source {
        ProfileSource::Profile(name) => load_profile(&name)?,
        ProfileSource::ProfileFromConfigsDump(name) => load_profile_from_configs_dump(&name)?,
    };
    dump("profile", &profile)?;

    let (mut paths, modes) = query_configs()?;
    paths.retain(|path| path.targetInfo.targetAvailable);

    let input_configs = aggregate_configs(paths, modes)?;
    dump("input-configs", &input_configs)?;

    let output_configs = solve_profile(profile, input_configs)?;
    dump("output-configs", &output_configs)?;

    let DisaggregatedConfigs { paths, modes, .. } = disaggregate_configs(output_configs);
    set_configs(args.action, &paths, &modes)?;

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ProfileSource {
    Profile(String),
    ProfileFromConfigsDump(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Action {
    Validate,
    Apply,
}

struct Args {
    profile_source: ProfileSource,
    action: Action,
}

fn parse_args() -> Result<Args> {
    let mut profile_source = None;
    let mut update_profile_source = |m| {
        if profile_source.is_some() {
            bail!("Cannot specify multiple profiles");
        }
        profile_source = Some(m);
        Ok(())
    };

    let mut action = None;
    let mut update_action = |a| {
        if action.is_some_and(|action| action != a) {
            bail!("Cannot both validate and apply");
        }
        action = Some(a);
        Ok(())
    };

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--apply" => update_action(Action::Apply)?,
            "--validate" => update_action(Action::Validate)?,
            "--profile" => {
                let Some(name) = args.next() else {
                    bail!("Missing profile name");
                };
                update_profile_source(ProfileSource::Profile(name))?;
            }
            "--profile-from-configs-dump" => {
                let Some(name) = args.next() else {
                    bail!("Missing configurations dump name");
                };
                update_profile_source(ProfileSource::ProfileFromConfigsDump(name))?;
            }
            _ => {
                bail!("Unexpected argument: {arg:?}");
            }
        }
    }

    let Some(profile_source) = profile_source else {
        bail!("Missing profile parameter");
    };

    let action = action.unwrap_or(Action::Validate);

    Ok(Args {
        profile_source,
        action,
    })
}

fn load_profile(profile_name: &str) -> Result<Profile> {
    load_json(profile_name, "profile.json")
}

fn load_profile_from_configs_dump(profile_name: &str) -> Result<Profile> {
    let configs: Configs = load_json(profile_name, "dump.json")?;

    let monitors = configs
        .into_iter()
        .filter(|path| {
            path.flags
                .contains(types::DISPLAYCONFIG_PATH_INFO_flag::ACTIVE)
        })
        .map(|path| {
            let source_mode = path.sourceMode.unwrap();

            Ok(Monitor {
                friendly_device_name: path.targetDeviceInfos.monitorFriendlyDeviceName,
                source_device_name: path.sourceDeviceInfos.viewGdiDeviceName,
                device_path: path.targetDeviceInfos.monitorDevicePath,
                width: source_mode.width,
                height: source_mode.height,
                pixel_format: source_mode.pixelFormat,
                position: source_mode.position,
                rotation: path.targetInfo.rotation,
                scaling: path.targetInfo.scaling,
                refresh_rate: path.targetInfo.refreshRate,
            })
        })
        .collect::<Result<_>>()?;

    Ok(monitors)
}

fn load_json<T: DeserializeOwned>(profile_name: &str, file_name: &str) -> Result<T> {
    let path = PathBuf::from_iter([DATA_DIR_PATH, "profiles", profile_name, file_name]);

    let json = fs::read_to_string(&path)
        .map_err(|error| anyhow!("Couldn't read file {}: {error}", path.display()))?;
    let content = serde_json::from_str(&json)?;

    Ok(content)
}

fn query_configs() -> Result<(
    Vec<types::DISPLAYCONFIG_PATH_INFO>,
    Vec<types::DISPLAYCONFIG_MODE_INFO>,
)> {
    let flags =
        QUERY_DISPLAY_CONFIG_FLAG::ALL_PATHS | QUERY_DISPLAY_CONFIG_FLAG::VIRTUAL_MODE_AWARE;

    let (paths, modes) = functions::query_display_config(flags)?;
    dump(
        "QueryDisplayConfig-parameter-and-outputs",
        &json!({
            "flags": flags,
            "paths": paths,
            "modes": modes,
        }),
    )?;

    Ok((paths, modes))
}

fn solve_profile(profile: Vec<Monitor>, configs: Configs) -> Result<Configs> {
    let mut solved_monitors = Vec::new();
    let mut unsolved_monitors = Vec::new();

    let mut source_device_ids_in_use = BTreeSet::new();

    // Try to reuse active paths
    for monitor in profile {
        let active_path = configs.iter().find(|path| {
            path.flags
                .contains(types::DISPLAYCONFIG_PATH_INFO_flag::ACTIVE)
                && path.targetDeviceInfos.monitorDevicePath == monitor.device_path
                && path.sourceDeviceInfos.viewGdiDeviceName == monitor.source_device_name
        });

        if let Some(path) = active_path {
            solved_monitors.push((monitor, path.clone()));
            source_device_ids_in_use.insert(path.sourceInfo.device_id);
        } else {
            unsolved_monitors.push(monitor);
        }
    }

    for monitor in unsolved_monitors {
        let path = configs.iter().find(|path| {
            !source_device_ids_in_use.contains(&path.sourceInfo.device_id)
                && path.targetDeviceInfos.monitorDevicePath == monitor.device_path
                && path.sourceDeviceInfos.viewGdiDeviceName == monitor.source_device_name
        });

        if let Some(path) = path {
            solved_monitors.push((monitor, path.clone()));
            source_device_ids_in_use.insert(path.sourceInfo.device_id);
        } else {
            bail!(
                "Can't find available path for monitor with device path and source device name {:?} {:?}",
                monitor.device_path,
                monitor.source_device_name,
            );
        }
    }

    drop(configs);

    let configs = solved_monitors
        .into_iter()
        .map(|(monitor, mut path)| {
            path.targetInfo.rotation = monitor.rotation;
            path.targetInfo.scaling = monitor.scaling;
            path.targetInfo.refreshRate = monitor.refresh_rate;

            path.sourceMode = Some(types::DISPLAYCONFIG_SOURCE_MODE {
                width: monitor.width,
                height: monitor.height,
                pixelFormat: monitor.pixel_format,
                position: monitor.position,
            });

            path.targetMode = Some(types::DISPLAYCONFIG_TARGET_MODE {
                targetVideoSignalInfo: types::DISPLAYCONFIG_VIDEO_SIGNAL_INFO {
                    pixelRate: 0,
                    hSyncFreq: types::DISPLAYCONFIG_RATIONAL {
                        Numerator: 0,
                        Denominator: 0,
                    },
                    vSyncFreq: monitor.refresh_rate,
                    activeSize: types::DISPLAYCONFIG_2DREGION {
                        cx: monitor.width,
                        cy: monitor.height,
                    },
                    totalSize: types::DISPLAYCONFIG_2DREGION { cx: 0, cy: 0 },
                    Anonymous: types::DISPLAYCONFIG_VIDEO_SIGNAL_INFO_0 {
                        videoStandard: types::D3DKMDT_VIDEO_SIGNAL_STANDARD::OTHER,
                    },
                    scanLineOrdering: types::DISPLAYCONFIG_SCANLINE_ORDERING::PROGRESSIVE,
                },
            });

            path.flags |= types::DISPLAYCONFIG_PATH_INFO_flag::ACTIVE;
            path
        })
        .collect();

    Ok(configs)
}

fn set_configs(
    action: Action,
    paths: &[types::DISPLAYCONFIG_PATH_INFO],
    modes: &[types::DISPLAYCONFIG_MODE_INFO],
) -> Result<()> {
    let (action, action_flag, success_message) = match action {
        Action::Validate => (
            "Validating",
            functions::SET_DISPLAY_CONFIG_FLAG::VALIDATE,
            "Configuration passed validation",
        ),
        Action::Apply => (
            "Applying",
            functions::SET_DISPLAY_CONFIG_FLAG::APPLY,
            "Configuration was SUCCESSFULY applied",
        ),
    };

    let flags = action_flag
        | (functions::SET_DISPLAY_CONFIG_FLAG::USE_SUPPLIED_DISPLAY_CONFIG
            | functions::SET_DISPLAY_CONFIG_FLAG::ALLOW_CHANGES
            | functions::SET_DISPLAY_CONFIG_FLAG::SAVE_TO_DATABASE
            | functions::SET_DISPLAY_CONFIG_FLAG::VIRTUAL_MODE_AWARE);
    dump(
        "SetDisplayConfig-parameters",
        &json!({
            "flags": flags,
            "paths": paths,
            "modes": modes,
        }),
    )?;

    eprintln!("{action} configuration...");
    functions::set_display_config(Some(paths), Some(modes), flags).into_unit_result()?;
    eprintln!("{success_message}");

    Ok(())
}

fn dump(name: &str, value: &impl Serialize) -> Result<()> {
    let mut file_path = PathBuf::from(DATA_DIR_PATH);
    file_path.push("apply-configs-dumps");
    file_path.push(name);
    file_path.set_extension("json");

    let file = File::create(&file_path)
        .map_err(|err| anyhow!("Failed to create {}: {err}", file_path.display()))?;

    serde_json::to_writer_pretty(file, value)?;

    Ok(())
}
