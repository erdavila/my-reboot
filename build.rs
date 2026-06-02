use std::io::BufRead as _;
use std::process::{Command, Output};

use anyhow::Result;
use chrono::Local;

fn main() -> Result<()> {
    generate_version_env_vars()?;

    #[cfg(windows)]
    if std::env::var("CARGO_CFG_TARGET_OS").is_ok_and(|var| var == "windows")
        && std::env::var("SKIP_WINDOWS_ICON").is_err()
    {
        winres::WindowsResource::new()
            .set_icon("assets/icon.ico")
            .compile()?;
    }

    Ok(())
}

fn generate_version_env_vars() -> Result<()> {
    println!("cargo::rustc-env=MY_REBOOT_NAME={}", env!("CARGO_PKG_NAME"));
    println!(
        "cargo::rustc-env=MY_REBOOT_VERSION={}",
        env!("CARGO_PKG_VERSION")
    );
    println!(
        "cargo::rustc-env=MY_REBOOT_TIMESTAMP={}",
        Local::now().to_rfc2822()
    );

    let (vcs, revision) = if let Some(jj_ids) = jj_ids() {
        ("JJ", jj_ids?)
    } else {
        ("Git", git_head()?)
    };
    println!("cargo::rustc-env=MY_REBOOT_VCS_REVISION={vcs}:{revision}");

    Ok(())
}

fn jj_ids() -> Option<Result<String>> {
    Command::new("jj")
        .args(["log", "-G"])
        .args(["-r", "@"])
        .args(["-T", r#"
            if(
                !empty,
                short_ids(self),
                parents.map(|p| short_ids(p)).join("\n")
            )
        "#])
        .args(["--config", r#"template-aliases.'short_ids(c)'='c.change_id().short() ++ " " ++ c.commit_id().short()'"#])
        .output()
        // It's ok to fail. `None` will be returned.
        .ok()
        .map(|output| first_line(&output))
}

fn git_head() -> Result<String> {
    let output = Command::new("git")
        .args(["describe", "--always", "--dirty"])
        .output()?;
    first_line(&output)
}

fn first_line(output: &Output) -> Result<String> {
    let line = output.stdout.lines().next().unwrap()?;
    Ok(line)
}
