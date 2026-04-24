use std::fs;

use anyhow::Result;

use crate::args::Action;

mod args;

fn main() -> Result<()> {
    let args = args::get()?;

    match args.action {
        Action::Save => save(&args.profile)?,
        Action::Apply => apply(&args.profile)?,
    }

    Ok(())
}

fn save(profile_path: &str) -> Result<()> {
    let file = fs::File::create(profile_path)?;
    let profile = display_profile_lib::get_profile()?;
    serde_json::to_writer_pretty(file, &profile)?;
    Ok(())
}

fn apply(profile_path: &str) -> Result<()> {
    let profile = fs::read_to_string(profile_path)?;
    let profile = serde_json::from_str(&profile)?;
    display_profile_lib::set_profile(&profile)?;
    Ok(())
}
