cfg_select! {
    windows => {
        fn main() -> std::io::Result<()> {
            if std::env::var("CARGO_CFG_TARGET_OS").is_ok_and(|var| var == "windows")
                && std::env::var("SKIP_WINDOWS_ICON").is_err()
            {
                winres::WindowsResource::new()
                    .set_icon("assets/icon.ico")
                    .compile()?;
            }

            Ok(())
        }
    },
    _ => {
        fn main() {}
    },
}
