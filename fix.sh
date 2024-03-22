#!/usr/bin/env bash 

cargo fmt --all
cargo fix --allow-dirty --allow-staged -q
cargo clippy --fix --allow-dirty --allow-staged
cargo sort -w
cargo-machete --fix --skip-target-dir