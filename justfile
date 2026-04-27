default: check test fmt clippy


# For cross-checks, the targets must be added with: rustup target add x86_64-pc-windows-gnu x86_64-unknown-linux-gnu
# Cross-checks the my-reboot package
[group('cross-check')]
cross-check:
  cargo check -p my-reboot --config "env.SKIP_WINDOWS_ICON='1'" \
    --target x86_64-pc-windows-gnu \
    --target x86_64-unknown-linux-gnu


[windows]
check: cross-check
  cargo check --workspace \
    --exclude my-reboot \
    --exclude display-profile-lib
  @# Checks the different combinations of features:
  cargo check -p display-profile-lib --no-default-features
  cargo check -p display-profile-lib --no-default-features --features serde

[linux]
check: cross-check


# Tests all packages
[windows]
test:
  cargo test --workspace

# Tests only the my-reboot package
[linux]
test:
  cargo test -p my-reboot


# Formats all packages
fmt:
  cargo +nightly fmt --all -- --config group_imports=StdExternalCrate --config imports_granularity=Module


# Runs clippy on all packages
[windows]
clippy:
  cargo clippy --all-targets --all-features --workspace

# Runs clippy only on my-reboot package
[linux]
clippy:
  cargo clippy --all-targets -p my-reboot


# Runs the display-profile CLI
[windows]
[group('binary execution')]
profile ACTION FILE:
  cargo run -q -p display-profile-cli -- {{ACTION}} {{FILE}}

[windows]
[group('binary execution')]
enum-display-devices:
  cargo run -q -p enum_display_devices

[windows]
[group('binary execution')]
is-windows-11-or-greater:
  cargo run -q -p is_windows_11_or_greater

[windows]
[group('binary execution')]
dump-configs *ARGS:
  cargo run -q -p display-profile-experiment --bin dump-configs -- {{ARGS}}

[windows]
[group('binary execution')]
apply-configs +ARGS:
  cargo run -q -p display-profile-experiment --bin apply-configs -- {{ARGS}}
