use anyhow::{Result, bail};
use display_profile::common::aggregate_configs;
use display_profile::output_format::OutputFormat;
use display_profile::win_api::{functions, types};

use crate::eq_ignoring_mode_idxs::eq_ignoring_mode_idxs;

mod eq_ignoring_mode_idxs;

const COMMON_QUERY_DISPLAY_CONFIG_FLAGS: functions::QUERY_DISPLAY_CONFIG_FLAG =
    functions::QUERY_DISPLAY_CONFIG_FLAG::VIRTUAL_MODE_AWARE;

fn main() -> Result<()> {
    let (check, output_format) = parse_args()?;

    let (paths, modes) = functions::query_display_config(
        functions::QUERY_DISPLAY_CONFIG_FLAG::ALL_PATHS | COMMON_QUERY_DISPLAY_CONFIG_FLAGS,
    )?;

    if check {
        check_active_paths(&paths, &modes)?;
    }

    let configs = aggregate_configs(paths, modes)?;
    output_format.write(&configs)?;

    Ok(())
}

fn parse_args() -> Result<(bool, OutputFormat)> {
    let mut check = true;
    let mut output_format = OutputFormat::JSON;

    let mut args = std::env::args().skip(1).peekable();
    loop {
        if args.next_if_eq("--check").is_some() {
            check = true;
            continue;
        }

        if args.next_if_eq("--no-check").is_some() {
            check = false;
            continue;
        }

        if let Some(fmt) = OutputFormat::from_args(&mut args) {
            output_format = fmt;
            continue;
        }

        if let Some(arg) = args.next() {
            bail!("Unrecognized argument: {arg:?}");
        }

        break;
    }

    Ok((check, output_format))
}

fn check_active_paths(
    all_paths: &[types::DISPLAYCONFIG_PATH_INFO],
    all_modes: &[types::DISPLAYCONFIG_MODE_INFO],
) -> Result<()> {
    // Active paths returned by the API
    let (active_paths, active_modes) = functions::query_display_config(
        functions::QUERY_DISPLAY_CONFIG_FLAG::ONLY_ACTIVE_PATHS | COMMON_QUERY_DISPLAY_CONFIG_FLAGS,
    )?;

    // Active paths filtered from all paths
    let filtered_active_paths: Vec<_> = all_paths
        .iter()
        .filter(|path| {
            path.flags
                .contains(types::DISPLAYCONFIG_PATH_INFO_flag::ACTIVE)
        })
        .collect();

    if filtered_active_paths.len() != active_paths.len() {
        bail!(
            "Number of active paths mismatch: {} from API, but {} filtered from all paths",
            active_paths.len(),
            filtered_active_paths.len(),
        );
    }

    for path in active_paths {
        let filtered_path = filtered_active_paths
            .iter()
            .find(|filtered_path| eq_ignoring_mode_idxs(&path, filtered_path));

        let Some(filtered_path) = filtered_path else {
            bail!(
                "Active path not found in all paths: {:?} {:?}",
                path.sourceInfo.device_id,
                path.targetInfo.device_id,
            );
        };

        check_source_modes(&path, filtered_path, &active_modes, all_modes)?;
        check_target_and_desktop_modes(&path, filtered_path, &active_modes, all_modes)?;
    }

    Ok(())
}

fn check_source_modes(
    left_path: &types::DISPLAYCONFIG_PATH_INFO,
    right_path: &types::DISPLAYCONFIG_PATH_INFO,
    left_modes: &[types::DISPLAYCONFIG_MODE_INFO],
    right_modes: &[types::DISPLAYCONFIG_MODE_INFO],
) -> Result<()> {
    assert_eq!(
        left_path.sourceInfo.device_id,
        right_path.sourceInfo.device_id,
    );

    let left_path_source_mode = left_path.source_mode_idx().map(|idx| &left_modes[idx]);
    let right_path_source_mode = right_path.source_mode_idx().map(|idx| &right_modes[idx]);
    compare_modes(
        left_path_source_mode,
        right_path_source_mode,
        left_path,
        "Source",
    )
}

fn check_target_and_desktop_modes(
    left_path: &types::DISPLAYCONFIG_PATH_INFO,
    right_path: &types::DISPLAYCONFIG_PATH_INFO,
    left_modes: &[types::DISPLAYCONFIG_MODE_INFO],
    right_modes: &[types::DISPLAYCONFIG_MODE_INFO],
) -> Result<()> {
    assert_eq!(
        left_path.targetInfo.device_id,
        right_path.targetInfo.device_id,
    );

    let left_path_target_mode = left_path.target_mode_idx().map(|idx| &left_modes[idx]);
    let right_path_target_mode = right_path.target_mode_idx().map(|idx| &right_modes[idx]);
    compare_modes(
        left_path_target_mode,
        right_path_target_mode,
        left_path,
        "Target",
    )?;

    let left_path_desktop_mode = left_path.desktop_mode_idx().map(|idx| &left_modes[idx]);
    let right_path_desktop_mode = right_path.desktop_mode_idx().map(|idx| &right_modes[idx]);
    compare_modes(
        left_path_desktop_mode,
        right_path_desktop_mode,
        left_path,
        "Desktop",
    )?;

    Ok(())
}

fn compare_modes(
    left: Option<&types::DISPLAYCONFIG_MODE_INFO>,
    right: Option<&types::DISPLAYCONFIG_MODE_INFO>,
    path: &types::DISPLAYCONFIG_PATH_INFO,
    descr: &'static str,
) -> Result<()> {
    match (left, right) {
        (None, None) => {}
        (Some(left_mode), Some(right_mode)) if left_mode == right_mode => {}
        _ => {
            bail!(
                "{descr} mode mismatch: {:?} {:?}",
                path.sourceInfo.device_id,
                path.targetInfo.device_id,
            );
        }
    }
    Ok(())
}
