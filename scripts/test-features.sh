#!/usr/bin/env bash
# All 16 primary-feature combinations, two compatibility aliases, and all-features.
set -euo pipefail
for features in '' serde strum rand 'serde strum' 'serde rand' 'strum rand' 'serde strum rand'; do
    for taxonomy in '' taxonomy; do
        cargo test --all-targets --no-default-features --features "$features $taxonomy"
        cargo test --doc --no-default-features --features "$features $taxonomy"
    done
done
for features in enum-iter enum-count; do
    cargo test --all-targets --no-default-features --features "$features"
    cargo test --doc --no-default-features --features "$features"
done
cargo test --all-targets --all-features
cargo test --doc --all-features
