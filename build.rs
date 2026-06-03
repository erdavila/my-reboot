use std::io::BufRead as _;
use std::process::{Command, Output};

use anyhow::Result;
use build_rs::input::{cargo_pkg_name, cargo_pkg_version};
use build_rs::output::rustc_env;
use chrono::Local;

fn main() -> Result<()> {
    build_rs::output::rerun_if_changed("src/");

    generate_version_env_vars()?;

    #[cfg(windows)]
    {
        use build_rs::output::rerun_if_env_changed;

        const WINDOWS_ICON_ENV_VAR: &str = "WINDOWS_ICON";

        rerun_if_env_changed(WINDOWS_ICON_ENV_VAR);
        if std::env::var(WINDOWS_ICON_ENV_VAR).is_ok_and(|value| value == "true") {
            winres::WindowsResource::new()
                .set_icon("assets/icon.ico")
                .compile()?;
        }
    }

    Ok(())
}

fn generate_version_env_vars() -> Result<()> {
    let (vcs, revision) = if let Some(jj_ids) = jj_ids() {
        ("JJ", jj_ids?)
    } else {
        ("Git", git_head()?)
    };

    rustc_env("MY_REBOOT_NAME", &cargo_pkg_name());
    rustc_env("MY_REBOOT_VERSION", &cargo_pkg_version());
    rustc_env("MY_REBOOT_TIMESTAMP", &Local::now().to_rfc2822());
    rustc_env("MY_REBOOT_VCS_REVISION", &format!("{vcs}:{revision}"));

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
