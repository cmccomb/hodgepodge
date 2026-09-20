#!/usr/bin/env bash
# Exhaustive compact-feature tests; representative full taxonomy tests and
# isolated optional-integration compilation checks avoid redundant huge builds.
set -euo pipefail
for features in '' serde strum rand 'serde strum' 'serde rand' 'strum rand' 'serde strum rand' enum-iter enum-count; do
    cargo test --all-targets --no-default-features --features "$features"
    cargo test --doc --no-default-features --features "$features"
done
for features in 'taxonomy serde' 'taxonomy strum' 'taxonomy rand'; do
    cargo check --all-targets --no-default-features --features "$features"
done
cargo test --all-targets --no-default-features --features taxonomy
cargo test --doc --no-default-features --features taxonomy
cargo test --all-targets --all-features
cargo test --doc --all-features
