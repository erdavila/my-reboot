#!/bin/bash
set -ex

cargo test --workspace
cargo +nightly fmt -- --config group_imports=StdExternalCrate --config imports_granularity=Module
cargo clippy --workspace --all-targets --no-deps
