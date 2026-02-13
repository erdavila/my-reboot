#!/bin/bash
set -ex

cargo test --workspace
cargo fmt
cargo clippy --workspace --all-targets --no-deps
