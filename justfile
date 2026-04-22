default: test fmt clippy

test:
  cargo test --workspace

fmt:
  cargo +nightly fmt --all -- --config group_imports=StdExternalCrate --config imports_granularity=Module

clippy:
  cargo clippy --workspace --all-targets --no-deps


dump-configs *ARGS:
  cargo run -q -p display-profile --bin dump-configs -- {{ARGS}}

apply-configs +ARGS:
  cargo run -q -p display-profile --bin apply-configs -- {{ARGS}}
