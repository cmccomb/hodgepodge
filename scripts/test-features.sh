#!/usr/bin/env bash
# Exercise standalone features, compatibility names, and documentation examples.
set -euo pipefail
for features in '' serde strum 'serde strum' enum-iter enum-count; do
    cargo test --all-targets --no-default-features --features "$features"
    cargo test --doc --no-default-features --features "$features"
done
cargo test --all-targets --all-features
cargo test --doc --all-features
