#!/bin/bash
set -ex

cargo fmt
cargo clippy --workspace
cargo test --workspace
