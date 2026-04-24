default: test fmt clippy


test:
  cargo test --workspace

fmt:
  cargo +nightly fmt --all -- --config group_imports=StdExternalCrate --config imports_granularity=Module

clippy:
  cargo clippy --workspace --all-targets --no-deps


profile ACTION FILE:
  cargo run -q -p display-profile-cli -- {{ACTION}} {{FILE}}

dump-configs *ARGS:
  cargo run -q -p display-profile-experiment --bin dump-configs -- {{ARGS}}

apply-configs +ARGS:
  cargo run -q -p display-profile-experiment --bin apply-configs -- {{ARGS}}


TMP-display-profile: TMP-display-profile-lib TMP-display-profile-cli

TMP-display-profile-lib:
  cargo check -p display-profile-lib
  cargo check -p display-profile-lib --features serde
  cargo clippy -p display-profile-lib --all-targets
  cargo clippy -p display-profile-lib --all-targets --features serde

TMP-display-profile-cli:
  cargo check -p display-profile-cli
  cargo clippy -p display-profile-cli --all-targets
