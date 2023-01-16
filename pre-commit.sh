#!/bin/bash
set -ex

cargo fmt
cargo clippy
cargo test
