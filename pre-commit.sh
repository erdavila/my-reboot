#!/bin/bash
set -ex

cargo test --workspace
cargo fmt
cargo clippy --workspace --no-deps
