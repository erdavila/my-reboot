default: test fmt clippy

test:
  cargo test --workspace

fmt:
  cargo +nightly fmt -- --config group_imports=StdExternalCrate --config imports_granularity=Module

clippy:
  cargo clippy --workspace --all-targets --no-deps
