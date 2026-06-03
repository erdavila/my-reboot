# check + test + fmt + clippy
default: check test fmt clippy


# For cross-checks, the targets must be added with: rustup target add x86_64-pc-windows-gnu x86_64-unknown-linux-gnu
# Cross-checks
check:
  cargo check --target x86_64-pc-windows-gnu
  cargo check --target x86_64-unknown-linux-gnu

test:
  cargo test

fmt:
  cargo +nightly fmt --all -- --config group_imports=StdExternalCrate --config imports_granularity=Module

clippy:
  cargo clippy --all-targets

run *ARGS:
  cargo run -q -p my-reboot -- {{ARGS}}
